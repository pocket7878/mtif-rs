use nom::{
    branch, bytes, character,
    combinator::{self, map},
    multi,
    sequence::{self, preceded},
    IResult,
};

pub fn parse_until_line_ending(input: &str) -> IResult<&str, &str> {
    sequence::terminated(
        bytes::complete::take_until("\n"),
        character::complete::newline,
    )(input)
}

pub fn take_n_digits<'a>(n: usize) -> impl FnMut(&'a str) -> IResult<&'a str, u32> {
    map(
        multi::many_m_n(n, n, character::complete::satisfy(|c| c.is_digit(10))),
        |digits: Vec<char>| {
            let num_str: String = digits.into_iter().collect();
            num_str
                .parse::<u32>()
                .expect("Invalid string, expected ASCII representation of a number")
        },
    )
}

pub fn parse_num_bool_flag(input: &str) -> IResult<&str, bool> {
    let flag_parser = sequence::terminated(
        branch::alt((bytes::complete::tag("0"), bytes::complete::tag("1"))),
        character::complete::newline,
    );
    let flag_to_bool = |status: &str| match status.to_ascii_lowercase().as_str() {
        "0" => false,
        "1" => true,
        _ => unreachable!(),
    };

    combinator::map(flag_parser, flag_to_bool)(input)
}

pub fn parse_date_value(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    let am_pm_parser = preceded(
        bytes::complete::tag(" "),
        branch::alt((bytes::complete::tag("AM"), bytes::complete::tag("PM"))),
    );
    let (input, (month, _, day, _, year, _, hour, _, minutes, _, seconds, am_pm)) =
        sequence::tuple((
            take_n_digits(2),
            bytes::complete::tag("/"),
            take_n_digits(2),
            bytes::complete::tag("/"),
            take_n_digits(4),
            bytes::complete::tag(" "),
            take_n_digits(2),
            bytes::complete::tag(":"),
            take_n_digits(2),
            bytes::complete::tag(":"),
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

pub fn parse_multiline_text(input: &str) -> IResult<&str, &str> {
    let multiline_data_separator: &str = "\n-----\n";
    sequence::terminated(
        bytes::complete::take_until(multiline_data_separator),
        bytes::complete::take(multiline_data_separator.len()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_until_line_ending() {
        assert_eq!(
            parse_until_line_ending("Foo Bar\nBaz Qux"),
            Ok(("Baz Qux", "Foo Bar"))
        )
    }

    #[test]
    fn test_take_n_digits() {
        assert_eq!(take_n_digits(2)("12"), Ok(("", 12)));
        assert_eq!(take_n_digits(2)("12/31"), Ok(("/31", 12)));
        assert_eq!(take_n_digits(4)("2004"), Ok(("", 2004)));
    }

    #[test]
    fn test_num_bool_flag() {
        assert_eq!(parse_num_bool_flag("0\n"), Ok(("", false)));
        assert_eq!(parse_num_bool_flag("1\n"), Ok(("", true)));
    }
}
