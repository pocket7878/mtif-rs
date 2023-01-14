use nom::{bytes, IResult};

use crate::model::MetaData;

use super::utils::parse_until_line_ending;

// CATEGORY: <text>\n
pub fn parse_category_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("CATEGORY: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaData::Category(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_category_data() {
        assert_eq!(
            parse_category_data("CATEGORY: Foo Bar\n"),
            Ok(("", MetaData::Category("Foo Bar".to_string())))
        );
    }
}
