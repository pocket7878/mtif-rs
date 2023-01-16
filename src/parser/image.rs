use nom::{
    bytes::{self},
    IResult,
};

use super::utils::parse_until_line_ending;
use super::MetaDataField;

// Hatena export format extension.
// IMAGE: <text>\n
pub fn parse_image_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("IMAGE: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaDataField::Author(text)))
}

#[cfg(test)]
mod tests {
    use super::MetaDataField;
    use super::*;

    #[test]
    fn test_parse_author_data() {
        assert_eq!(
            parse_image_data("IMAGE: Foo Bar\n"),
            Ok(("", MetaDataField::Author("Foo Bar")))
        );
    }
}
