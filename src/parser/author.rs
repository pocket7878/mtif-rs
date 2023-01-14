use nom::{
    bytes::{self},
    IResult,
};

use super::utils::parse_until_line_ending;
use crate::model::MetaData;

// AUTHOR: <text>\n
pub fn parse_author_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("AUTHOR: ")(input)?;
    let (input, text) = parse_until_line_ending(input)?;

    Ok((input, MetaData::Author(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MetaData;

    #[test]
    fn test_parse_author_data() {
        assert_eq!(
            parse_author_data("AUTHOR: Foo Bar\n"),
            Ok(("", MetaData::Author("Foo Bar".to_string())))
        );
    }
}
