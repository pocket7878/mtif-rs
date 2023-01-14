use crate::model::MetaData;
use nom::{
    branch,
    bytes::{self},
    character::streaming::newline,
    combinator::{self},
    sequence::{self, preceded, terminated},
    IResult,
};

use super::utils::take_n_digits;

fn parse_date_value(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    let am_pm_parser = preceded(
        bytes::streaming::tag(" "),
        branch::alt((bytes::streaming::tag("AM"), bytes::streaming::tag("PM"))),
    );
    let (input, (month, _, day, _, year, _, hour, _, minutes, _, seconds, am_pm)) =
        sequence::tuple((
            take_n_digits(2),
            bytes::streaming::tag("/"),
            take_n_digits(2),
            bytes::streaming::tag("/"),
            take_n_digits(4),
            bytes::streaming::tag(" "),
            take_n_digits(2),
            bytes::streaming::tag(":"),
            take_n_digits(2),
            bytes::streaming::tag(":"),
            take_n_digits(2),
            combinator::opt(am_pm_parser),
        ))(input)?;

    let date =
        time::Date::from_calendar_date(year as i32, (month as u8).try_into().unwrap(), day as u8)
            .expect("Invalid date");
    let time =
        time::Time::from_hms(hour as u8, minutes as u8, seconds as u8).expect("Invalid time");
    if let Some(am_pm) = am_pm {
        let time = match am_pm {
            "AM" => time,
            "PM" => time + time::Duration::hours(12),
            _ => unreachable!(),
        };
        Ok((input, time::PrimitiveDateTime::new(date, time)))
    } else {
        Ok((input, time::PrimitiveDateTime::new(date, time)))
    }
}

// DATE: MM/DD/YYYY HH:MM:SS AM|PM?\n
pub fn parse_date_data(input: &str) -> IResult<&str, MetaData> {
    let (input, _) = bytes::streaming::tag("DATE: ")(input)?;
    let (input, date) = terminated(parse_date_value, newline)(input)?;

    Ok((input, MetaData::Date(date)))
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
                MetaData::Date(time::macros::datetime!(2012-12-31 12:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 12:34:56 AM\n"),
            Ok((
                "",
                MetaData::Date(time::macros::datetime!(2012-12-31 12:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 01:34:56 PM\n"),
            Ok((
                "",
                MetaData::Date(time::macros::datetime!(2012-12-31 13:34:56))
            ))
        );
        assert_eq!(
            parse_date_data("DATE: 12/31/2012 13:34:56\n"),
            Ok((
                "",
                MetaData::Date(time::macros::datetime!(2012-12-31 13:34:56))
            ))
        );
    }
}
