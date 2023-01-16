//!
//! # Movable Type Import Format Parser
//!
//! ```rust
//! use mtif::{MTIFParser, MTIFEntry};
//!
//! let parser = MTIFParser::new();
//! let contents = std::fs::read_to_string("./example/example.txt").unwrap();
//! let entries = parser.parse(&contents).unwrap();
//! dbg!(entries);
//! ```
//!
//! Output:
//! ```text
//! [
//! 	MTIFEntry {
//! 		metadata: MetaData {
//! 				author: Some(
//! 						"Foo Bar",
//! 				),
//! 				title: Some(
//! 						"A dummy title",
//! 				),
//! 				basename: Some(
//! 						"a-dummy-title",
//! 				),
//! 				status: None,
//! 				allow_comments: None,
//! 				allow_pings: None,
//! 				convert_breaks: None,
//! 				primary_category: Some(
//! 						"Media",
//! 				),
//! 				category: [
//! 						"News",
//! 				],
//! 				date: 2002-01-31 15:31:05.0,
//! 				no_entry: false,
//! 				tags: [],
//! 				image: None,
//! 		},
//! 		body: Some(
//! 				"This is the body.\n\nAnother paragraph here.\n\nAnother paragraph here.",
//! 		),
//! 		extended_body: Some(
//! 				"Here is some more text.\n\nAnother paragraph here.\n\nAnother paragraph here.",
//! 		),
//! 		excerpt: None,
//! 		keywords: None,
//! 		comments: [
//! 				Comment {
//! 						author: Some(
//! 								"Foo",
//! 						),
//! 						email: None,
//! 						url: None,
//! 						ip: None,
//! 						date: Some(
//! 								2002-01-31 15:47:06.0,
//! 						),
//! 						text: "This is\nthe body of this comment.",
//! 				},
//! 				Comment {
//! 						author: Some(
//! 								"Bar",
//! 						),
//! 						email: Some(
//! 								"me@bar.com",
//! 						),
//! 						url: None,
//! 						ip: Some(
//! 								"205.66.1.32",
//! 						),
//! 						date: Some(
//! 								2002-02-01 4:02:07.0,
//! 						),
//! 						text: "This is the body of\nanother comment. It goes\nup to here.",
//! 				},
//! 		],
//! 		pings: [
//! 				Ping {
//! 						title: Some(
//! 								"My Entry",
//! 						),
//! 						url: Some(
//! 								"http://www.foo.com/old/2002/08/",
//! 						),
//! 						ip: Some(
//! 								"206.22.1.53",
//! 						),
//! 						date: Some(
//! 								2002-08-05 16:09:12.0,
//! 						),
//! 						blog_name: Some(
//! 								"My Weblog",
//! 						),
//! 						text: "This is the start of my\nentry, and here it...",
//! 				},
//! 		],
//! 	},
//! 	MTIFEntry {
//! 		metadata: MetaData {
//! 				author: Some(
//! 						"Baz Quux",
//! 				),
//! 				title: Some(
//! 						"Here is a new entry",
//! 				),
//! 				basename: Some(
//! 						"here-is-a-new-entry",
//! 				),
//! 				status: None,
//! 				allow_comments: None,
//! 				allow_pings: None,
//! 				convert_breaks: None,
//! 				primary_category: None,
//! 				category: [
//! 						"Politics",
//! 				],
//! 				date: 2002-01-31 3:31:05.0,
//! 				no_entry: false,
//! 				tags: [],
//! 				image: None,
//! 		},
//! 		body: Some(
//! 				"This is the body of the second entry. It can\nconsist of multiple lines.",
//! 		),
//! 		extended_body: None,
//! 		excerpt: Some(
//! 				"See, this entry does not have an extended piece; but\nit does have an excerpt. It is special.",
//! 		),
//! 		keywords: None,
//! 		comments: [
//! 				Comment {
//! 						author: Some(
//! 								"Quux",
//! 						),
//! 						email: None,
//! 						url: Some(
//! 								"http://www.quux.com/",
//! 						),
//! 						ip: None,
//! 						date: Some(
//! 								2002-01-31 16:23:01.0,
//! 						),
//! 						text: "Here is the first comment on this entry.",
//! 				},
//! 		],
//! 		pings: [],
//! 	},
//! ]

pub mod model;
mod parser;

pub use model::MTIFEntry;
use model::{Comment, MetaData, Ping};

pub struct MTIFParser {}

impl MTIFParser {
    pub fn new() -> Self {
        MTIFParser {}
    }

    pub fn parse<'a>(&self, input: &'a str) -> Result<Vec<MTIFEntry<'a>>, String> {
        let (_, entries) = parser::parse_mtif(input).map_err(|e| e.to_string())?;

        let entries = entries
            .iter()
            .map(|e| self.build_mtif_entry_from_raw_mtif_entry(e))
            .collect();

        entries
    }

    fn build_mtif_entry_from_raw_mtif_entry<'a>(
        &self,
        raw_mtif_entry: &parser::RawMTIFEntry<'a>,
    ) -> Result<MTIFEntry<'a>, String> {
        let metadata = self.build_metadata_from_raw_entry(raw_mtif_entry)?;

        Ok(MTIFEntry {
            metadata,
            body: raw_mtif_entry.multiline_data.iter().find_map(|m| match m {
                parser::MultiLineField::Body(body) => Some(*body),
                _ => None,
            }),
            extended_body: raw_mtif_entry.multiline_data.iter().find_map(|m| match m {
                parser::MultiLineField::ExtendedBody(extended_body) => Some(*extended_body),
                _ => None,
            }),
            excerpt: raw_mtif_entry.multiline_data.iter().find_map(|m| match m {
                parser::MultiLineField::Excerpt(excerpt) => Some(*excerpt),
                _ => None,
            }),
            keywords: raw_mtif_entry.multiline_data.iter().find_map(|m| match m {
                parser::MultiLineField::Keywords(keywords) => Some(*keywords),
                _ => None,
            }),
            comments: raw_mtif_entry
                .multiline_data
                .iter()
                .filter_map(|m| match m {
                    parser::MultiLineField::Comment {
                        author,
                        email,
                        url,
                        ip,
                        date,
                        text,
                    } => Some(Comment {
                        author: *author,
                        email: *email,
                        url: *url,
                        ip: *ip,
                        date: *date,
                        text: *text,
                    }),
                    _ => None,
                })
                .collect(),
            pings: raw_mtif_entry
                .multiline_data
                .iter()
                .filter_map(|m| match m {
                    parser::MultiLineField::Ping {
                        title,
                        url,
                        ip,
                        date,
                        blog_name,
                        text,
                    } => Some(Ping {
                        title: *title,
                        url: *url,
                        ip: *ip,
                        date: *date,
                        blog_name: *blog_name,
                        text: *text,
                    }),
                    _ => None,
                })
                .collect(),
        })
    }

    fn build_metadata_from_raw_entry<'a>(
        &self,
        raw_mtif_entry: &parser::RawMTIFEntry<'a>,
    ) -> Result<MetaData<'a>, String> {
        let ref raw_metadata = raw_mtif_entry.metadata;
        let date_value = raw_metadata.iter().find_map(|m| match m {
            parser::MetaDataField::Date(date) => Some(*date),
            _ => None,
        });
        if date_value.is_none() {
            return Err("Date is required".to_string());
        }
        Ok(MetaData {
            author: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::Author(author) => Some(*author),
                _ => None,
            }),
            title: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::Title(title) => Some(*title),
                _ => None,
            }),
            basename: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::BaseName(basename) => Some(*basename),
                _ => None,
            }),
            status: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::Status(status) => Some(*status),
                _ => None,
            }),
            allow_comments: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::AllowComments(allow_comments) => Some(*allow_comments),
                _ => None,
            }),
            allow_pings: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::AllowPings(allow_pings) => Some(*allow_pings),
                _ => None,
            }),
            convert_breaks: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::ConvertBreaks(convert_breaks) => Some(*convert_breaks),
                _ => None,
            }),
            primary_category: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::PrimaryCategory(primary_category) => Some(*primary_category),
                _ => None,
            }),
            category: raw_metadata
                .iter()
                .filter_map(|m| match m {
                    parser::MetaDataField::Category(category) => Some(*category),
                    _ => None,
                })
                .collect(),
            date: date_value.unwrap(),
            no_entry: raw_metadata.iter().any(|m| match m {
                parser::MetaDataField::NoEntry => true,
                _ => false,
            }),
            tags: raw_metadata
                .iter()
                .find_map(|m| match m {
                    parser::MetaDataField::Tags(tags) => Some(tags.clone()),
                    _ => None,
                })
                .unwrap_or(vec![]),
            image: raw_metadata.iter().find_map(|m| match m {
                parser::MetaDataField::Image(image) => Some(*image),
                _ => None,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mtif() {
        let parser = MTIFParser::new();
        let contents = std::fs::read_to_string("./example/example.txt").unwrap();
        let entries = parser.parse(&contents).unwrap();
        insta::assert_debug_snapshot!(entries);
    }
}
