use nom::{
    branch,
    bytes::{self, streaming::tag},
    character::{
        self,
        streaming::{newline, satisfy},
    },
    multi::{many0, separated_list0},
    sequence, IResult,
};

use crate::model::MetaData;

fn parse_quoted_tag_entry(input: &str) -> IResult<&str, String> {
    let (input, _) = character::streaming::char('"')(input)?;
    let (input, contents) = many0(satisfy(|c| c != '"' && c != '\n'))(input)?;
    let (input, _) = character::streaming::char('"')(input)?;

    return Ok((input, contents.into_iter().collect()));
}

fn parse_non_quoted_tag_entry(input: &str) -> IResult<&str, String> {
    let is_non_quoted_tag_contents = |c: char| {
        let result = !c.is_whitespace() && c != '\n' && c != ',';
        return result;
    };
    let (input, value) = many0(satisfy(|c| is_non_quoted_tag_contents(c)))(input)?;

    return Ok((input, value.into_iter().collect()));
}

pub fn parse_tags_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("TAGS: ")(input)?;
    let tag_entry_parser = branch::alt((parse_quoted_tag_entry, parse_non_quoted_tag_entry));
    let (input, tag_entries) =
        sequence::terminated(separated_list0(tag(","), tag_entry_parser), newline)(input)?;

    return Ok((
        input,
        MetaData::Tags(tag_entries.iter().map(|s| s.to_string()).collect()),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quoted_tag_entry() {
        assert_eq!(
            parse_quoted_tag_entry(r#""foo bar""#),
            Ok(("", "foo bar".to_string()))
        );
    }

    #[test]
    fn test_parse_tags_data() {
        assert_eq!(
            parse_tags_data("TAGS: \"Movable Type\",foo,bar\n"),
            Ok((
                "",
                MetaData::Tags(vec![
                    "Movable Type".to_string(),
                    "foo".to_string(),
                    "bar".to_string()
                ])
            ))
        );
    }
}
