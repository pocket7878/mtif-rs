use nom::{bytes, IResult};

use super::MetaDataField;

use super::utils::parse_num_bool_flag;

// ALLOW COMMENTS: 0|1\n
pub fn parse_allow_comments_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("ALLOW COMMENTS: ")(input)?;
    let (input, flag) = parse_num_bool_flag(input)?;

    Ok((input, MetaDataField::AllowComments(flag)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_allow_comments_data() {
        assert_eq!(
            parse_allow_comments_data("ALLOW COMMENTS: 0\n"),
            Ok(("", MetaDataField::AllowComments(false)))
        );
        assert_eq!(
            parse_allow_comments_data("ALLOW COMMENTS: 1\n"),
            Ok(("", MetaDataField::AllowComments(true)))
        );
    }
}
