use nom::{bytes, IResult};

use super::{utils::parse_multiline_text, MultiLineField};

pub fn parse_keywords_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("KEYWORDS:\n")(input)?;
    let (input, keywords) = parse_multiline_text(input)?;

    Ok((input, MultiLineField::Keywords(keywords.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keywords_data() {
        assert_eq!(
            parse_keywords_data("KEYWORDS:\nFoo Bar\nBaz Qux\n\n-----\n"),
            Ok((
                "",
                MultiLineField::Keywords("Foo Bar\nBaz Qux\n\n".to_string())
            ))
        );
    }
}
