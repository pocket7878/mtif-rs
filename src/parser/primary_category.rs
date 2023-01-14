use nom::{bytes, IResult};

use crate::model::MetaData;

use super::utils::parse_until_line_ending;

// PRIMARY CATEGORY: <text>\n
pub fn parse_primary_category_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("PRIMARY CATEGORY: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaData::PrimaryCategory(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primary_category_data() {
        assert_eq!(
            parse_primary_category_data("PRIMARY CATEGORY: Foo Bar\n"),
            Ok(("", MetaData::PrimaryCategory("Foo Bar".to_string())))
        );
    }
}
