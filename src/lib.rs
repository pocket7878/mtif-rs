mod parser;
mod model;

struct Entry {
    author: Option<String>
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Draft,
    Publish,
    Future
}

