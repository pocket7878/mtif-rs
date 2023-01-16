use super::MetaDataField;
use nom::{
    bytes::{self},
    IResult,
};

use super::utils::parse_until_line_ending;

// BASENAME: <text>\n
pub fn parse_basename_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("BASENAME: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaDataField::BaseName(text)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basename_data() {
        assert_eq!(
            parse_basename_data("BASENAME: filename\n"),
            Ok(("", MetaDataField::BaseName("filename")))
        );
    }
}
