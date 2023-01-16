use nom::{bytes, IResult};

use super::MetaDataField;

use super::utils::parse_until_line_ending;

// PRIMARY CATEGORY: <text>\n
pub fn parse_primary_category_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("PRIMARY CATEGORY: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaDataField::PrimaryCategory(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primary_category_data() {
        assert_eq!(
            parse_primary_category_data("PRIMARY CATEGORY: Foo Bar\n"),
            Ok(("", MetaDataField::PrimaryCategory("Foo Bar".to_string())))
        );
    }
}
