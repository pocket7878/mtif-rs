use nom::{bytes, IResult};

use super::utils::parse_multiline_text;

pub fn parse_extended_body_data(input: &str) -> IResult<&str, &str> {
    let (input, _) = bytes::streaming::tag("EXTENDED BODY:\n")(input)?;
    parse_multiline_text(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extended_body_data() {
        assert_eq!(
            parse_extended_body_data("EXTENDED BODY:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", "Foo Bar\nBaz Qux\n\n"))
        );
    }
}
