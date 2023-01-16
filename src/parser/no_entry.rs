use nom::{bytes, IResult};

use super::MetaDataField;

// NO ENTRY: 1\n
pub fn parse_no_entry_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("NO ENTRY: 1\n")(input)?;

    Ok((input, MetaDataField::NoEntry))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_entry_data() {
        assert_eq!(
            parse_no_entry_data("NO ENTRY: 1\n"),
            Ok(("", MetaDataField::NoEntry))
        );
    }
}
