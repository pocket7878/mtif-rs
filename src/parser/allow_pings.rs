use nom::{bytes, IResult};

use super::MetaDataField;

use super::utils::parse_num_bool_flag;

// ALLOW PINGS: 0|1\n
pub fn parse_allow_pings_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("ALLOW PINGS: ")(input)?;
    let (input, flag) = parse_num_bool_flag(input)?;

    Ok((input, MetaDataField::AllowPings(flag)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_allow_pings_data() {
        assert_eq!(
            parse_allow_pings_data("ALLOW PINGS: 0\n"),
            Ok(("", MetaDataField::AllowPings(false)))
        );
        assert_eq!(
            parse_allow_pings_data("ALLOW PINGS: 1\n"),
            Ok(("", MetaDataField::AllowPings(true)))
        );
    }
}
