mod model;
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
