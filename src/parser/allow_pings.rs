use nom::{bytes, IResult};

use crate::model::MetaData;

use super::utils::parse_num_bool_flag;

// ALLOW PINGS: 0|1\n
pub fn parse_allow_pings_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("ALLOW PINGS: ")(input)?;
    let (input, flag) = parse_num_bool_flag(input)?;

    Ok((input, MetaData::AllowPings(flag)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_allow_pings_data() {
        assert_eq!(
            parse_allow_pings_data("ALLOW PINGS: 0\n"),
            Ok(("", MetaData::AllowPings(false)))
        );
        assert_eq!(
            parse_allow_pings_data("ALLOW PINGS: 1\n"),
            Ok(("", MetaData::AllowPings(true)))
        );
    }
}
