#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Draft,
    Publish,
    Future,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConvertBreaks {
    None,
    Convert,
    Markdown,
    MarkdownWithSmartypants,
    RichText,
    Textile2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MetaData<'a> {
    pub author: Option<&'a str>,
    pub title: Option<&'a str>,
    pub basename: Option<&'a str>,
    pub status: Option<Status>,
    pub allow_comments: Option<bool>,
    pub allow_pings: Option<bool>,
    pub convert_breaks: Option<ConvertBreaks>,
    pub primary_category: Option<&'a str>,
    pub category: Vec<&'a str>,
    pub date: time::PrimitiveDateTime,
    pub no_entry: bool,
    pub tags: Vec<&'a str>,
    pub image: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comment<'a> {
    pub author: Option<&'a str>,
    pub email: Option<&'a str>,
    pub url: Option<&'a str>,
    pub ip: Option<&'a str>,
    pub date: Option<time::PrimitiveDateTime>,
    pub text: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ping<'a> {
    pub title: Option<&'a str>,
    pub url: Option<&'a str>,
    pub ip: Option<&'a str>,
    pub date: Option<time::PrimitiveDateTime>,
    pub blog_name: Option<&'a str>,
    pub text: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MTIFEntry<'a> {
    pub metadata: MetaData<'a>,
    pub body: Option<&'a str>,
    pub extended_body: Option<&'a str>,
    pub excerpt: Option<&'a str>,
    pub keywords: Option<&'a str>,
    pub comments: Vec<Comment<'a>>,
    pub pings: Vec<Ping<'a>>,
}
