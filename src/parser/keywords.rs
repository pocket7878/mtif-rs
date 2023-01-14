use nom::{bytes, IResult};

use super::utils::parse_multiline_text;

pub fn parse_keywords_data(input: &str) -> IResult<&str, &str> {
    let (input, _) = bytes::streaming::tag("KEYWORDS:\n")(input)?;
    parse_multiline_text(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keywords_data() {
        assert_eq!(
            parse_keywords_data("KEYWORDS:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", "Foo Bar\nBaz Qux\n\n"))
        );
    }
}
