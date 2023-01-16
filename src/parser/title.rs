use nom::{
    bytes::{self},
    IResult,
};

use super::MetaDataField;

use super::utils::parse_until_line_ending;

// TITLE: <text>\n
pub fn parse_title_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("TITLE: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaDataField::Title(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_title_data() {
        assert_eq!(
            parse_title_data("TITLE: A dummy title\n"),
            Ok(("", MetaDataField::Title("A dummy title".to_string())))
        );
    }
}
