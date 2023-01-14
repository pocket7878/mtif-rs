use nom::{bytes, IResult};

use crate::model::MetaData;

use super::utils::parse_num_bool_flag;

// ALLOW COMMENTS: 0|1\n
pub fn parse_allow_comments_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("ALLOW COMMENTS: ")(input)?;
    let (input, flag) = parse_num_bool_flag(input)?;

    Ok((input, MetaData::AllowComments(flag)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_allow_comments_data() {
        assert_eq!(
            parse_allow_comments_data("ALLOW COMMENTS: 0\n"),
            Ok(("", MetaData::AllowComments(false)))
        );
        assert_eq!(
            parse_allow_comments_data("ALLOW COMMENTS: 1\n"),
            Ok(("", MetaData::AllowComments(true)))
        );
    }
}
