# mtif

[![mtif at crates.io](https://img.shields.io/crates/v/mtif.svg)](https://crates.io/crates/mtif)
[![mtif at docs.rs](https://docs.rs/mtif/badge.svg)](https://docs.rs/mtif)

[Movable Type Import Format](https://movabletype.org/documentation/appendices/import-export-format.html) parser in Rust.

# Usage

```rust
use mtif::{MTIFParser, MTIFEntry};

let parser = MTIFParser::new();
let contents = std::fs::read_to_string("./example/example.txt").unwrap();
let entries = parser.parse(&contents).unwrap();
dbg!(entries)
```

Output (omitted):

```rust
[
    MTIFEntry {
        metadata: MetaData {
            author: Some(
                "Foo Bar",
            ),
            title: Some(
                "A dummy title",
            ),
            basename: Some(
                "a-dummy-title",
            ),
            status: None,
            allow_comments: None,
            allow_pings: None,
            convert_breaks: None,
            primary_category: Some(
                "Media",
            ),
            category: [
                "News",
            ],
            date: 2002-01-31 15:31:05.0,
            no_entry: false,
            tags: [],
            image: None,
        },
        body: Some(
            "This is the body.\n\nAnother paragraph here.\n\nAnother paragraph here.",
        ),
        extended_body: Some(
            "Here is some more text.\n\nAnother paragraph here.\n\nAnother paragraph here.",
        ),
        excerpt: None,
        keywords: None,
        comments: [
            Comment {
                author: Some(
                    "Foo",
                ),
                email: None,
                url: None,
                ip: None,
                date: Some(
                    2002-01-31 15:47:06.0,
                ),
                text: "This is\nthe body of this comment.",
            },
            Comment {
                author: Some(
                    "Bar",
                ),
                email: Some(
                    "me@bar.com",
                ),
                url: None,
                ip: Some(
                    "205.66.1.32",
                ),
                date: Some(
                    2002-02-01 4:02:07.0,
                ),
                text: "This is the body of\nanother comment. It goes\nup to here.",
            },
        ],
        pings: [
            Ping {
                title: Some(
                    "My Entry",
                ),
                url: Some(
                    "http://www.foo.com/old/2002/08/",
                ),
                ip: Some(
                    "206.22.1.53",
                ),
                date: Some(
                    2002-08-05 16:09:12.0,
                ),
                blog_name: Some(
                    "My Weblog",
                ),
                text: "This is the start of my\nentry, and here it...",
            },
        ],
    },
		... omitted
]
```

## Licence

See the [LICENCE](LICENCE).
