use nom::{branch, bytes, character, combinator, sequence, IResult};

use crate::model::Status;

use super::MetaDataField;

// STATUS: Draft|Publish|Future\n
pub fn parse_status_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("STATUS: ")(input)?;
    let status_tag_parser = sequence::terminated(
        branch::alt((
            bytes::complete::tag_no_case("Draft"),
            bytes::complete::tag_no_case("Publish"),
            bytes::complete::tag_no_case("Future"),
        )),
        character::complete::newline,
    );
    let status_str_to_enum = |status: &str| match status.to_ascii_lowercase().as_str() {
        "draft" => Status::Draft,
        "publish" => Status::Publish,
        "future" => Status::Future,
        _ => unreachable!(),
    };

    let (input, status) = combinator::map(status_tag_parser, status_str_to_enum)(input)?;

    Ok((input, MetaDataField::Status(status)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_data() {
        assert_eq!(
            parse_status_data("STATUS: dRaFT\n"),
            Ok(("", MetaDataField::Status(Status::Draft)))
        );
        assert_eq!(
            parse_status_data("STATUS: PuBLiSh\n"),
            Ok(("", MetaDataField::Status(Status::Publish)))
        );
        assert_eq!(
            parse_status_data("STATUS: FUTURE\n"),
            Ok(("", MetaDataField::Status(Status::Future)))
        );
    }
}
