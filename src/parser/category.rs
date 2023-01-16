use nom::{bytes, IResult};

use super::MetaDataField;

use super::utils::parse_until_line_ending;

// CATEGORY: <text>\n
pub fn parse_category_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("CATEGORY: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaDataField::Category(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_category_data() {
        assert_eq!(
            parse_category_data("CATEGORY: Foo Bar\n"),
            Ok(("", MetaDataField::Category("Foo Bar".to_string())))
        );
    }
}
