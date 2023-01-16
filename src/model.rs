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

#[derive(Debug, PartialEq, Eq)]
pub struct MetaData<'a> {
    pub author: &'a str,
    pub title: Option<&'a str>,
    pub basename: Option<&'a str>,
    pub status: Option<Status>,
    pub allow_comments: Option<bool>,
    pub allow_pings: Option<bool>,
    pub convert_breaks: Option<ConvertBreaks>,
    pub primary_category: &'a str,
    pub category: Vec<&'a str>,
    pub date: time::PrimitiveDateTime,
    pub no_entry: bool,
    pub tags: Vec<&'a str>,
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
    title: Option<&'a str>,
    url: Option<&'a str>,
    ip: Option<&'a str>,
    date: Option<time::PrimitiveDateTime>,
    blog_name: Option<&'a str>,
    text: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MTIFEntry<'a> {
    pub metadata: MetaData<'a>,
    pub body: &'a str,
    pub extended_body: &'a str,
    pub keywords: &'a str,
    pub comments: Vec<Comment<'a>>,
    pub pings: Vec<Ping<'a>>,
}
