#[derive(Debug, PartialEq, Eq)]
pub struct MetaData {
    pub author: String,
    pub title: Option<String>,
    pub basename: Option<String>,
    pub status: Option<Status>,
    pub allow_comments: Option<bool>,
    pub allow_pings: Option<bool>,
    pub convert_breaks: Option<ConvertBreaks>,
    pub primary_category: String,
    pub category: Vec<String>,
    pub date: time::PrimitiveDateTime,
    pub no_entry: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Draft,
    Publish,
    Future,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConvertBreaks {
    None,
    Convert,
    Markdown,
    MarkdownWithSmartypants,
    RichText,
    Textile2,
}
