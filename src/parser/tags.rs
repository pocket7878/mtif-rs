use nom::{
    branch,
    bytes::{
        self,
        complete::{tag, take_while},
    },
    character::{
        self,
        complete::{newline, satisfy},
    },
    multi::{many0, separated_list0},
    sequence, IResult,
};

use super::MetaDataField;

fn parse_quoted_tag_entry(input: &str) -> IResult<&str, &str> {
    let (input, _) = character::complete::char('"')(input)?;
    let (input, contents) = take_while(|c| c != '"' && c != '\n')(input)?;
    let (input, _) = character::complete::char('"')(input)?;

    return Ok((input, contents));
}

fn parse_non_quoted_tag_entry(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_while(|c: char| !c.is_whitespace() && c != '\n' && c != ',')(input)?;

    return Ok((input, value));
}

pub fn parse_tags_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("TAGS: ")(input)?;
    let tag_entry_parser = branch::alt((parse_quoted_tag_entry, parse_non_quoted_tag_entry));
    let (input, tag_entries) =
        sequence::terminated(separated_list0(tag(","), tag_entry_parser), newline)(input)?;

    return Ok((input, MetaDataField::Tags(tag_entries)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quoted_tag_entry() {
        assert_eq!(parse_quoted_tag_entry(r#""foo bar""#), Ok(("", "foo bar")));
    }

    #[test]
    fn test_parse_tags_data() {
        assert_eq!(
            parse_tags_data("TAGS: \"Movable Type\",foo,bar\n"),
            Ok(("", MetaDataField::Tags(vec!["Movable Type", "foo", "bar"])))
        );
    }
}
