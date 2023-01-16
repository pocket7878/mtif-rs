use nom::{
    branch, bytes, character::complete::newline, multi::many0, sequence::terminated, IResult,
};

use super::{
    utils::{parse_date_value, parse_multiline_text, parse_until_line_ending},
    MultiLineField,
};

enum PingField<'a> {
    Title(&'a str),
    Url(&'a str),
    Ip(&'a str),
    BlogName(&'a str),
    Date(time::PrimitiveDateTime),
}

fn parse_title_field(input: &str) -> IResult<&str, PingField> {
    let (input, _) = bytes::complete::tag("TITLE: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, PingField::Title(contents)))
}

fn parse_url_field(input: &str) -> IResult<&str, PingField> {
    let (input, _) = bytes::complete::tag("URL: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, PingField::Url(contents)))
}

fn parse_ip_field(input: &str) -> IResult<&str, PingField> {
    let (input, _) = bytes::complete::tag("IP: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, PingField::Ip(contents)))
}

fn parse_blog_name_field(input: &str) -> IResult<&str, PingField> {
    let (input, _) = bytes::complete::tag("BLOG NAME: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, PingField::BlogName(contents)))
}

fn parse_date_field(input: &str) -> IResult<&str, PingField> {
    let (input, _) = bytes::complete::tag("DATE: ")(input)?;
    let (input, date) = terminated(parse_date_value, newline)(input)?;

    Ok((input, PingField::Date(date)))
}

fn parse_ping_fields(input: &str) -> IResult<&str, Vec<PingField>> {
    many0(branch::alt((
        parse_title_field,
        parse_url_field,
        parse_ip_field,
        parse_blog_name_field,
        parse_date_field,
    )))(input)
}

pub fn parse_ping_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("PING:\n")(input)?;
    let (input, fields) = parse_ping_fields(input)?;
    let (input, text) = parse_multiline_text(input)?;

    let ping = MultiLineField::Ping {
        title: fields.iter().find_map(|f| match f {
            PingField::Title(title) => Some(*title),
            _ => None,
        }),
        url: fields.iter().find_map(|f| match f {
            PingField::Url(url) => Some(*url),
            _ => None,
        }),
        ip: fields.iter().find_map(|f| match f {
            PingField::Ip(ip) => Some(*ip),
            _ => None,
        }),
        blog_name: fields.iter().find_map(|f| match f {
            PingField::BlogName(blog_name) => Some(*blog_name),
            _ => None,
        }),
        date: fields.iter().find_map(|f| match f {
            PingField::Date(date) => Some(*date),
            _ => None,
        }),
        text: text,
    };

    Ok((input, ping))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ping_without_field_data() {
        let target_text = "PING:\nTITLE: sample title\nURL: https://example.com/\nIP: 192.0.2.0\nBLOG NAME: sample blog name\nDATE: 12/31/2023 01:34:56 PM\nFoo Bar\nBaz Qux\n\n-----\n";
        let (rest, result) = parse_ping_data(target_text).unwrap();
        if rest != "" {
            panic!("rest is not empty: {}", rest);
        }
        if let MultiLineField::Ping {
            title,
            url,
            ip,
            blog_name,
            date,
            text,
        } = result
        {
            assert_eq!(title, Some("sample title"));
            assert_eq!(url, Some("https://example.com/"));
            assert_eq!(ip, Some("192.0.2.0"));
            assert_eq!(blog_name, Some("sample blog name"));
            assert_eq!(date, Some(time::macros::datetime!(2023-12-31 13:34:56)));
            assert_eq!(text, "Foo Bar\nBaz Qux\n");
        }
    }
}
