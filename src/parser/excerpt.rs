use nom::{bytes, IResult};

use super::utils::parse_multiline_text;

pub fn parse_excerpt_data(input: &str) -> IResult<&str, &str> {
    let (input, _) = bytes::streaming::tag("EXCERPT:\n")(input)?;
    parse_multiline_text(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_excerpt_data() {
        assert_eq!(
            parse_excerpt_data("EXCERPT:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", "Foo Bar\nBaz Qux\n\n"))
        );
    }
}
