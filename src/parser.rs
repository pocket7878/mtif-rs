/*
 * https://www.movabletype.jp/documentation/mt7/appendices/export-import-format/
 */

mod allow_comments;
mod allow_pings;
mod author;
mod basename;
mod body;
mod category;
mod comment;
mod convert_breaks;
mod date;
mod excerpt;
mod extended_body;
mod keywords;
mod no_entry;
mod ping;
mod primary_category;
mod status;
mod tags;
mod title;
mod utils;

use crate::model::{ConvertBreaks, MetaData, Status};

use nom::{
    branch,
    bytes::{self},
    character::complete::{newline, satisfy},
    combinator::eof,
    error::ErrorKind,
    multi::{self},
    sequence::{self, terminated},
    IResult,
};

const multiline_data_separator: &str = "-----\n";

#[derive(Debug, PartialEq, Eq)]
pub enum MetaDataField<'a> {
    Author(&'a str),
    Title(&'a str),
    BaseName(&'a str),
    Status(Status),
    AllowComments(bool),
    AllowPings(bool),
    ConvertBreaks(ConvertBreaks),
    Category(&'a str),
    PrimaryCategory(&'a str),
    Tags(Vec<&'a str>),
    Date(time::PrimitiveDateTime),
    NoEntry,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultiLineField<'a> {
    Body(&'a str),
    ExtendedBody(&'a str),
    Excerpt(&'a str),
    Keywords(&'a str),
    Comment {
        author: Option<&'a str>,
        email: Option<&'a str>,
        url: Option<&'a str>,
        ip: Option<&'a str>,
        date: Option<time::PrimitiveDateTime>,
        text: &'a str,
    },
    Ping {
        title: Option<&'a str>,
        url: Option<&'a str>,
        ip: Option<&'a str>,
        date: Option<time::PrimitiveDateTime>,
        blog_name: Option<&'a str>,
        text: &'a str,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct MTIFEntry<'a> {
    pub metadata: Vec<MetaDataField<'a>>,
    pub multiline_data: Vec<MultiLineField<'a>>,
}

// Meta data parsers
fn parse_metadata_section(input: &str) -> IResult<&str, Vec<MetaDataField>> {
    let metadata_parser = branch::alt((
        author::parse_author_data,
        title::parse_title_data,
        basename::parse_basename_data,
        status::parse_status_data,
        allow_comments::parse_allow_comments_data,
        allow_pings::parse_allow_pings_data,
        convert_breaks::parse_convert_breaks_data,
        primary_category::parse_primary_category_data,
        category::parse_category_data,
        date::parse_date_data,
        tags::parse_tags_data,
        no_entry::parse_no_entry_data,
    ));

    sequence::terminated(
        multi::many0(metadata_parser),
        bytes::complete::tag("-----\n"),
    )(input)
}

// Multi-line data parsers
fn parse_multiline_data_section(input: &str) -> IResult<&str, Vec<MultiLineField>> {
    let multiline_data_parser = branch::alt((
        body::parse_body_data,
        extended_body::parse_extended_body_data,
        excerpt::parse_excerpt_data,
        keywords::parse_keywords_data,
        comment::parse_comment_data,
        ping::parse_ping_data,
    ));

    multi::many0(multiline_data_parser)(input)
}

// MTIF parser
fn parse_mtif_entry(input: &str) -> IResult<&str, MTIFEntry> {
    let (input, metadata) = parse_metadata_section(input)?;
    let (input, multiline_data) = parse_multiline_data_section(input)?;
    let (input, _) = bytes::complete::tag("--------")(input)?;

    Ok((
        input,
        MTIFEntry {
            metadata,
            multiline_data,
        },
    ))
}

pub fn parse_mtif(input: &str) -> IResult<&str, Vec<MTIFEntry>> {
    terminated(multi::separated_list0(newline, parse_mtif_entry), eof)(input)
}

/*
 * Multi-line data parser
 */
#[cfg(test)]
mod tests {
    use nom_bufreader::{bufreader::BufReader, Parse};
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_metadata_section() {
        assert_eq!(
            parse_metadata_section("AUTHOR: Foo Bar\nTITLE: Baz Qux\n-----\n"),
            Ok((
                "",
                vec![
                    MetaDataField::Author("Foo Bar"),
                    MetaDataField::Title("Baz Qux")
                ]
            ))
        );
    }

    #[test]
    fn test_parse_mtif() {
        let contents = fs::read_to_string("./example/example.txt").unwrap();
        dbg!(&contents);
        let (rest, entries) = parse_mtif(&contents).unwrap();
        assert_eq!(rest, "");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].metadata.len(), 6);
        assert_eq!(entries[0].multiline_data.len(), 5);
        assert_eq!(entries[1].metadata.len(), 5);
        assert_eq!(entries[1].multiline_data.len(), 3);
    }
}
