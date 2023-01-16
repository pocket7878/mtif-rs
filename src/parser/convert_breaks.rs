use nom::{branch, bytes, character, combinator, sequence, IResult};

use crate::model::ConvertBreaks;

use super::MetaDataField;

// CONVERT BREAKS: 0|1|markdown_with_smartypants|markdown|richtext|textile_2\n
pub fn parse_convert_breaks_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("CONVERT BREAKS: ")(input)?;
    let value_parser = sequence::terminated(
        branch::alt((
            bytes::complete::tag("0"),
            bytes::complete::tag("1"),
            // markdownの方が先にマッチしてしまうと困るので、長い方からマッチさせる
            bytes::complete::tag("markdown_with_smartypants"),
            bytes::complete::tag("markdown"),
            bytes::complete::tag("richtext"),
            bytes::complete::tag("textile_2"),
        )),
        character::complete::newline,
    );
    let value_to_enum = |value: &str| match value.to_ascii_lowercase().as_str() {
        "0" => ConvertBreaks::None,
        "1" => ConvertBreaks::Convert,
        "markdown" => ConvertBreaks::Markdown,
        "markdown_with_smartypants" => ConvertBreaks::MarkdownWithSmartypants,
        "richtext" => ConvertBreaks::RichText,
        "textile_2" => ConvertBreaks::Textile2,
        _ => unreachable!(),
    };

    let (input, convert_breaks) = combinator::map(value_parser, value_to_enum)(input)?;

    Ok((input, MetaDataField::ConvertBreaks(convert_breaks)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_convert_breaks_data() {
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: 0\n"),
            Ok(("", MetaDataField::ConvertBreaks(ConvertBreaks::None)))
        );
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: 1\n"),
            Ok(("", MetaDataField::ConvertBreaks(ConvertBreaks::Convert)))
        );
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: markdown\n"),
            Ok(("", MetaDataField::ConvertBreaks(ConvertBreaks::Markdown)))
        );
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: markdown_with_smartypants\n"),
            Ok((
                "",
                MetaDataField::ConvertBreaks(ConvertBreaks::MarkdownWithSmartypants)
            ))
        );
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: richtext\n"),
            Ok(("", MetaDataField::ConvertBreaks(ConvertBreaks::RichText)))
        );
        assert_eq!(
            parse_convert_breaks_data("CONVERT BREAKS: textile_2\n"),
            Ok(("", MetaDataField::ConvertBreaks(ConvertBreaks::Textile2)))
        );
    }
}
