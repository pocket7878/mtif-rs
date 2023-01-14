/*
 * https://www.movabletype.jp/documentation/mt7/appendices/export-import-format/
 */

use super::model::{MetaData, Status, ConvertBreaks};
use nom::{IResult, character, multi, combinator, bytes, branch, sequence};

const multiline_data_separator: &str = "-----\n";

/*
 * Meta data parsers
 */
fn parse_until_line_ending(input: &str) -> IResult<&str, &str> {
	sequence::terminated(bytes::streaming::take_until("\n"), character::streaming::newline)(input)
}

pub fn parse_author_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("AUTHOR: ")(input)?;
	let (input, text) = parse_until_line_ending(input)?;

	Ok((input, MetaData::Author(text.to_string())))
}

pub fn parse_title_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("TITLE: ")(input)?;
	let (input, text) = parse_until_line_ending(input)?;

	Ok((input, MetaData::Title(text.to_string())))
}

pub fn parse_basename_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("BASENAME: ")(input)?;
	let (input, text) = parse_until_line_ending(input)?;

	Ok((input, MetaData::BaseName(text.to_string())))
}

pub fn parse_status_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("STATUS: ")(input)?;
	let status_tag_parser = sequence::terminated(
		branch::alt((
			bytes::streaming::tag_no_case("Draft"),
			bytes::streaming::tag_no_case("Publish"),
			bytes::streaming::tag_no_case("Future")
		)),
		character::streaming::newline
	);
	let status_str_to_enum = |status: &str| {
			match status.to_ascii_lowercase().as_str() {
					"draft" => Status::Draft,
					"publish" => Status::Publish,
					"future" => Status::Future,
					_ => unreachable!()
			}
	};

	let (input, status) = combinator::map(status_tag_parser, status_str_to_enum)(input)?;

	Ok((input, MetaData::Status(status)))
}

fn parse_num_bool_flag(input: &str) -> IResult<&str, bool> {
	let flag_parser = sequence::terminated(
		branch::alt((
			bytes::streaming::tag("0"),
			bytes::streaming::tag("1")
		)),
		character::streaming::newline
	);
	let flag_to_bool = |status: &str| {
			match status.to_ascii_lowercase().as_str() {
					"0" => false,
					"1" => true,
					_ => unreachable!()
			}
	};

	combinator::map(flag_parser, flag_to_bool)(input)
}

pub fn parse_allow_comments_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("ALLOW COMMENTS: ")(input)?;
	let (input, flag) = parse_num_bool_flag(input)?;

	Ok((input, MetaData::AllowComments(flag)))
}

pub fn parse_allow_pings_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("ALLOW PINGS: ")(input)?;
	let (input, flag) = parse_num_bool_flag(input)?;

	Ok((input, MetaData::AllowPings(flag)))
}

pub fn parse_convert_breaks_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("CONVERT BREAKS: ")(input)?;
	let value_parser = sequence::terminated(
		branch::alt((
			bytes::streaming::tag("0"),
			bytes::streaming::tag("1"),
			// markdownの方が先にマッチしてしまうと困るので、長い方からマッチさせる
			bytes::streaming::tag("markdown_with_smartypants"),
			bytes::streaming::tag("markdown"),
			bytes::streaming::tag("richtext"),
			bytes::streaming::tag("textile_2")
		)),
		character::streaming::newline,
	);
	let value_to_enum = |value: &str| {
			match value.to_ascii_lowercase().as_str() {
					"0" => ConvertBreaks::None,
					"1" => ConvertBreaks::Convert,
					"markdown" => ConvertBreaks::Markdown,
					"markdown_with_smartypants" => ConvertBreaks::MarkdownWithSmartypants,
					"richtext" => ConvertBreaks::RichText,
					"textile_2" => ConvertBreaks::Textile2,
					_ => unreachable!()
			}
	};

	let (input, convert_breaks) = combinator::map(value_parser, value_to_enum)(input)?;

	Ok((input, MetaData::ConvertBreaks(convert_breaks)))
}

pub fn parse_primary_category_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("PRIMARY CATEGORY: ")(input)?;
	let (input, text) = parse_until_line_ending(input)?;

	Ok((input, MetaData::PrimaryCategory(text.to_string())))
}

pub fn parse_category_data(input: &str) -> IResult<&str, MetaData> {
	let (input, _) = bytes::streaming::tag("CATEGORY: ")(input)?;
	let (input, text) = parse_until_line_ending(input)?;

	Ok((input, MetaData::Category(text.to_string())))
}

pub fn parse_metadata_section(input: &str) -> IResult<&str, Vec<MetaData>> {
	let metadata_parser = branch::alt((
		parse_author_data,
		parse_title_data,
		parse_basename_data,
		parse_status_data,
		parse_allow_comments_data,
		parse_allow_pings_data,
		parse_convert_breaks_data,
		parse_primary_category_data,
		parse_category_data,
	));

	sequence::terminated(multi::many0(metadata_parser), bytes::streaming::tag("-----\n"))(input)
}

/*
 * Multi-line data parser
 */
pub fn parse_multiline_text(input: &str) -> IResult<&str, &str> {
	sequence::terminated(
		bytes::streaming::take_until(multiline_data_separator), 
		bytes::streaming::take(multiline_data_separator.len())
	)(input)
}

pub fn parse_body_data(input: &str) -> IResult<&str, &str> {
	let (input, _) = bytes::streaming::tag("BODY:\n")(input)?;
	parse_multiline_text(input)
}

pub fn parse_extended_body_data(input: &str) -> IResult<&str, &str> {
	let (input, _) = bytes::streaming::tag("EXTENDED BODY:\n")(input)?;
	parse_multiline_text(input)
}

pub fn parse_excerpt_data(input: &str) -> IResult<&str, &str> {
	let (input, _) = bytes::streaming::tag("EXCERPT:\n")(input)?;
	parse_multiline_text(input)
}

pub fn parse_keywords_data(input: &str) -> IResult<&str, &str> {
	let (input, _) = bytes::streaming::tag("KEYWORDS:\n")(input)?;
	parse_multiline_text(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_until_line_ending() {
			assert_eq!(parse_until_line_ending("Foo Bar\nBaz Qux"), Ok(("Baz Qux", "Foo Bar")))
	}

	#[test]
	fn test_parse_author_data() {
			assert_eq!(parse_author_data("AUTHOR: Foo Bar\n"), Ok(("", MetaData::Author("Foo Bar".to_string()))));
	}

	#[test]
	fn test_parse_title_data() {
			assert_eq!(parse_title_data("TITLE: A dummy title\n"), Ok(("", MetaData::Title("A dummy title".to_string()))));
	}

	#[test]
	fn test_parse_basename_data() {
			assert_eq!(parse_basename_data("BASENAME: filename\n"), Ok(("", MetaData::BaseName("filename".to_string()))));
	}

	#[test]
	fn test_parse_status_data() {
			assert_eq!(parse_status_data("STATUS: dRaFT\n"), Ok(("", MetaData::Status(Status::Draft))));
			assert_eq!(parse_status_data("STATUS: PuBLiSh\n"), Ok(("", MetaData::Status(Status::Publish))));
			assert_eq!(parse_status_data("STATUS: FUTURE\n"), Ok(("", MetaData::Status(Status::Future))));
	}

	#[test]
	fn test_parse_allow_comments_data() {
			assert_eq!(parse_allow_comments_data("ALLOW COMMENTS: 0\n"), Ok(("", MetaData::AllowComments(false))));
			assert_eq!(parse_allow_comments_data("ALLOW COMMENTS: 1\n"), Ok(("", MetaData::AllowComments(true))));
	}

	#[test]
	fn test_parse_allow_pings_data() {
			assert_eq!(parse_allow_pings_data("ALLOW PINGS: 0\n"), Ok(("", MetaData::AllowPings(false))));
			assert_eq!(parse_allow_pings_data("ALLOW PINGS: 1\n"), Ok(("", MetaData::AllowPings(true))));
	}

	#[test]
	fn test_parse_convert_breaks_data() {
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: 0\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::None))));
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: 1\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::Convert))));
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: markdown\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::Markdown))));
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: markdown_with_smartypants\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::MarkdownWithSmartypants))));
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: richtext\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::RichText))));
			assert_eq!(parse_convert_breaks_data("CONVERT BREAKS: textile_2\n"), Ok(("", MetaData::ConvertBreaks(ConvertBreaks::Textile2))));
	}

	#[test]
	fn test_parse_primary_category_data() {
			assert_eq!(parse_primary_category_data("PRIMARY CATEGORY: Foo Bar\n"), Ok(("", MetaData::PrimaryCategory("Foo Bar".to_string()))));
	}

	#[test]
	fn test_parse_category_data() {
			assert_eq!(parse_category_data("CATEGORY: Foo Bar\n"), Ok(("", MetaData::Category("Foo Bar".to_string()))));
	}

	#[test]
	fn test_parse_metadata_section() {
		assert_eq!(
			parse_metadata_section("AUTHOR: Foo Bar\nTITLE: Baz Qux\n-----\n"),
			Ok(("", vec![MetaData::Author("Foo Bar".to_string()), MetaData::Title("Baz Qux".to_string())])));
	}

	#[test]
	fn test_parse_body_data() {
			assert_eq!(parse_body_data("BODY:\nFoo Bar\nBaz Qux\n\n-----\n"), Ok(("", "Foo Bar\nBaz Qux\n\n")));
	}

	#[test]
	fn test_parse_extended_body_data() {
			assert_eq!(parse_extended_body_data("EXTENDED BODY:\nFoo Bar\nBaz Qux\n\n-----\n"), Ok(("", "Foo Bar\nBaz Qux\n\n")));
	}

	#[test]
	fn test_parse_excerpt_data() {
			assert_eq!(parse_excerpt_data("EXCERPT:\nFoo Bar\nBaz Qux\n\n-----\n"), Ok(("", "Foo Bar\nBaz Qux\n\n")));
	}

	#[test]
	fn test_parse_keywords_data() {
			assert_eq!(parse_keywords_data("KEYWORDS:\nFoo Bar\nBaz Qux\n\n-----\n"), Ok(("", "Foo Bar\nBaz Qux\n\n")));
	}
}