use nom::{bytes, IResult};

use super::utils::parse_multiline_text;

pub fn parse_body_data(input: &str) -> IResult<&str, &str> {
    let (input, _) = bytes::streaming::tag("BODY:\n")(input)?;
    parse_multiline_text(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_body_data() {
        assert_eq!(
            parse_body_data("BODY:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", "Foo Bar\nBaz Qux\n\n"))
        );
    }
}
