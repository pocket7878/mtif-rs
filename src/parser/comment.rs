use std::ops::Mul;

use nom::{
    branch, bytes, character::complete::newline, multi::many0, sequence::terminated, IResult,
};

use super::{
    date::parse_date_data,
    utils::{parse_date_value, parse_multiline_text, parse_until_line_ending},
    MultiLineField,
};

enum CommentField<'a> {
    Email(&'a str),
    Url(&'a str),
    Author(&'a str),
    Ip(&'a str),
    Date(time::PrimitiveDateTime),
}

fn parse_email_field(input: &str) -> IResult<&str, CommentField> {
    let (input, _) = bytes::complete::tag("EMAIL: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, CommentField::Email(contents)))
}

fn parse_url_field(input: &str) -> IResult<&str, CommentField> {
    let (input, _) = bytes::complete::tag("URL: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, CommentField::Url(contents)))
}

fn parse_author_field(input: &str) -> IResult<&str, CommentField> {
    let (input, _) = bytes::complete::tag("AUTHOR: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, CommentField::Author(contents)))
}

fn parse_ip_field(input: &str) -> IResult<&str, CommentField> {
    let (input, _) = bytes::complete::tag("IP: ")(input)?;
    let (input, contents) = parse_until_line_ending(input)?;

    Ok((input, CommentField::Ip(contents)))
}

fn parse_date_field(input: &str) -> IResult<&str, CommentField> {
    let (input, _) = bytes::complete::tag("DATE: ")(input)?;
    let (input, date) = terminated(parse_date_value, newline)(input)?;

    Ok((input, CommentField::Date(date)))
}

fn parse_comment_fields(input: &str) -> IResult<&str, Vec<CommentField>> {
    many0(branch::alt((
        parse_email_field,
        parse_url_field,
        parse_author_field,
        parse_ip_field,
        parse_date_field,
    )))(input)
}

pub fn parse_comment_data(input: &str) -> IResult<&str, MultiLineField> {
    let (input, _) = bytes::complete::tag("COMMENT:\n")(input)?;
    let (input, fields) = parse_comment_fields(input)?;
    let (input, text) = parse_multiline_text(input)?;

    let comment = MultiLineField::Comment {
        author: fields.iter().find_map(|f| match f {
            CommentField::Author(author) => Some(*author),
            _ => None,
        }),
        email: fields.iter().find_map(|f| match f {
            CommentField::Email(email) => Some(*email),
            _ => None,
        }),
        url: fields.iter().find_map(|f| match f {
            CommentField::Url(url) => Some(*url),
            _ => None,
        }),
        ip: fields.iter().find_map(|f| match f {
            CommentField::Ip(ip) => Some(*ip),
            _ => None,
        }),
        date: fields.iter().find_map(|f| match f {
            CommentField::Date(date) => Some(*date),
            _ => None,
        }),
        text: text,
    };

    Ok((input, comment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment_without_field_data() {
        let target_text = "COMMENT:\nAUTHOR: author\nEMAIL: sample@example.com\nURL: https://example.com/\nIP: 192.0.2.0\nDATE: 12/31/2023 01:34:56 PM\nFoo Bar\nBaz Qux\n\n-----\n";
        let (rest, result) = parse_comment_data(target_text).unwrap();
        if rest != "" {
            panic!("rest is not empty: {}", rest);
        }
        if let MultiLineField::Comment {
            author,
            email,
            url,
            ip,
            date,
            text,
        } = result
        {
            assert_eq!(author, Some("author"));
            assert_eq!(email, Some("sample@example.com"));
            assert_eq!(url, Some("https://example.com/"));
            assert_eq!(ip, Some("192.0.2.0"));
            assert_eq!(date, Some(time::macros::datetime!(2023-12-31 13:34:56)));
            assert_eq!(text, "Foo Bar\nBaz Qux\n");
        }
    }
}
