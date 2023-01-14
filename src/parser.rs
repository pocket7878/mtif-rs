/*
 * https://www.movabletype.jp/documentation/mt7/appendices/export-import-format/
 */

mod allow_comments;
mod allow_pings;
mod author;
mod basename;
mod body;
mod category;
mod convert_breaks;
mod date;
mod excerpt;
mod extended_body;
mod keywords;
mod primary_category;
mod status;
mod tags;
mod title;
mod utils;

use super::model::MetaData;
use nom::{
    branch,
    bytes::{self},
    multi::{self},
    sequence::{self},
    IResult,
};

const multiline_data_separator: &str = "-----\n";

/*
 * Meta data parsers
 */
pub fn parse_metadata_section(input: &str) -> IResult<&str, Vec<MetaData>> {
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
    ));

    sequence::terminated(
        multi::many0(metadata_parser),
        bytes::streaming::tag("-----\n"),
    )(input)
}

/*
 * Multi-line data parser
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_section() {
        assert_eq!(
            parse_metadata_section("AUTHOR: Foo Bar\nTITLE: Baz Qux\n-----\n"),
            Ok((
                "",
                vec![
                    MetaData::Author("Foo Bar".to_string()),
                    MetaData::Title("Baz Qux".to_string())
                ]
            ))
        );
    }
}
