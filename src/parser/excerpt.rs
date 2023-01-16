use nom::{bytes, IResult};

use super::{utils::parse_multiline_text, MultiLineField};

pub fn parse_excerpt_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("EXCERPT:\n")(input)?;
    let (input, text) = parse_multiline_text(input)?;

    Ok((input, MultiLineField::Excerpt(text)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_excerpt_data() {
        assert_eq!(
            parse_excerpt_data("EXCERPT:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok(("", MultiLineField::Excerpt("Foo Bar\nBaz Qux\n\n")))
        );
    }
}
