use nom::{branch, bytes, character, combinator, sequence, IResult};

use crate::model::{MetaData, Status};

// STATUS: Draft|Publish|Future\n
pub fn parse_status_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("STATUS: ")(input)?;
    let status_tag_parser = sequence::terminated(
        branch::alt((
            bytes::streaming::tag_no_case("Draft"),
            bytes::streaming::tag_no_case("Publish"),
            bytes::streaming::tag_no_case("Future"),
        )),
        character::streaming::newline,
    );
    let status_str_to_enum = |status: &str| match status.to_ascii_lowercase().as_str() {
        "draft" => Status::Draft,
        "publish" => Status::Publish,
        "future" => Status::Future,
        _ => unreachable!(),
    };

    let (input, status) = combinator::map(status_tag_parser, status_str_to_enum)(input)?;

    Ok((input, MetaData::Status(status)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_data() {
        assert_eq!(
            parse_status_data("STATUS: dRaFT\n"),
            Ok(("", MetaData::Status(Status::Draft)))
        );
        assert_eq!(
            parse_status_data("STATUS: PuBLiSh\n"),
            Ok(("", MetaData::Status(Status::Publish)))
        );
        assert_eq!(
            parse_status_data("STATUS: FUTURE\n"),
            Ok(("", MetaData::Status(Status::Future)))
        );
    }
}
