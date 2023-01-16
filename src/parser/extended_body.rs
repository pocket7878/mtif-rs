use nom::{bytes, IResult};

use super::{utils::parse_multiline_text, MultiLineField};

pub fn parse_extended_body_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("EXTENDED BODY:\n")(input)?;
    let (input, text) = parse_multiline_text(input)?;

    Ok((input, MultiLineField::ExtendedBody(text.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extended_body_data() {
        assert_eq!(
            parse_extended_body_data("EXTENDED BODY:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok((
                "",
                MultiLineField::ExtendedBody("Foo Bar\nBaz Qux\n\n".to_string())
            ))
        );
    }
}
