#[derive(Debug, PartialEq, Eq)]
pub enum MetaData {
		Author(String),
		Title(String),
		BaseName(String),
		Status(Status),
		AllowComments(bool),
		AllowPings(bool),
		ConvertBreaks(ConvertBreaks),
		Category(String),
		PrimaryCategory(String),
		Tags(Vec<String>),
		Date(time::OffsetDateTime),
		CF50BaseName(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Draft,
    Publish,
    Future
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConvertBreaks {
	None,
	Convert,
	Markdown,
	MarkdownWithSmartypants,
	RichText,
	Textile2
}