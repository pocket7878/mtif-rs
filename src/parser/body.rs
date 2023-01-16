use nom::{bytes, IResult};

use super::{utils::parse_multiline_text, MultiLineField};

pub fn parse_body_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("BODY:\n")(input)?;
    let (input, text) = parse_multiline_text(input)?;

    Ok((input, MultiLineField::Body(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_body_data() {
        assert_eq!(
            parse_body_data("BODY:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", MultiLineField::Body("Foo Bar\nBaz Qux\n\n".to_string())))
        );
    }
}
