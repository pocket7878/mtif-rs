use super::{utils::parse_date_value, MetaDataField};
use nom::{
    bytes::{self},
    character::complete::newline,
    sequence::terminated,
    IResult,
};

// DATE: MM/DD/YYYY HH:MM:SS AM|PM?\n
pub fn parse_date_data(input: &str) -> IResult<&str, MetaDataField> {
    let (input, _) = bytes::complete::tag("DATE: ")(input)?;
    let (input, date) = terminated(parse_date_value, newline)(input)?;

    Ok((input, MetaDataField::Date(date)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_data() {
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 12:34:56\n"),
            Ok((
                "",
                MetaDataField::Date(time::macros::datetime!(2012-12-31 12:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 12:34:56 AM\n"),
            Ok((
                "",
                MetaDataField::Date(time::macros::datetime!(2012-12-31 12:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 01:34:56 PM\n"),
            Ok((
                "",
                MetaDataField::Date(time::macros::datetime!(2012-12-31 13:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 13:34:56\n"),
            Ok((
                "",
                MetaDataField::Date(time::macros::datetime!(2012-12-31 13:34:56))
            ))
        );
    }
}
