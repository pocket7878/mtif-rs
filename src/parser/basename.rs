use crate::model::MetaData;
use nom::{
    bytes::{self},
    IResult,
};

use super::utils::parse_until_line_ending;

// BASENAME: <text>\n
pub fn parse_basename_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("BASENAME: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaData::BaseName(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basename_data() {
        assert_eq!(
            parse_basename_data("BASENAME: filename\n"),
            Ok(("", MetaData::BaseName("filename".to_string())))
        );
    }
}
