use nom::{
    bytes::{self},
    IResult,
};

use crate::model::MetaData;

use super::utils::parse_until_line_ending;

// TITLE: <text>\n
pub fn parse_title_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("TITLE: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaData::Title(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_title_data() {
        assert_eq!(
            parse_title_data("TITLE: A dummy title\n"),
            Ok(("", MetaData::Title("A dummy title".to_string())))
        );
    }
}
