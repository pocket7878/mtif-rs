mod model;
mod parser;

struct Entry {
    author: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Draft,
    Publish,
    Future,
}
