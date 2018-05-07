// #[macro_use]
// extern crate failure;
// #[macro_use]
// extern crate lazy_static;
// extern crate regex;
// extern crate sac;
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

extern crate pulldown_cmark;

use std::borrow::Cow;

use pulldown_cmark::{Event, Options, Parser, OPTION_ENABLE_TABLES};
use pulldown_cmark::html::push_html;

// use sac::value::Value;
// use sac::value::integer::Integer;

fn main() {
    // let u = Value::Untyped("foo".into());
    // println!("{:?}", u);

    // println!("{:?}", Value::String("bar".into()));
    // println!("{:?}", Value::Integer(Integer(1)));

    let markdown = r#"
lorem *ipsum* [dolor] **sit amet** <i>oo</i>

<p>lala</p>

```sh
ls
```
"#;
    let mut html = String::new();

    // let mut parser_opts = Options::empty();

    let parser = Parser::new(markdown);
    let events = Parser::new(markdown).filter_map(|ev| match ev {
        // Escape inline html
        Event::Html(html) => Some(format!("Found HTML inside Markdown: {:?}", html)),
        Event::InlineHtml(html) => Some(format!("Found inline HTML inside Markdown: {:?}", html)),
        _ => None,
    });

    for e in events {
        println!("{:?}", e);
    }

    push_html(&mut html, parser);

    println!("{:?}", html);
}
