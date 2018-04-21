#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate sac;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use sac::item;

fn main() {
    let raw = r#"{"non-escapes":"❤bar\u2764", "escapes\u006F": "\u006F\t\u001F\n", "path": "//foo/bar/far"}"#;
    let itemr = item::from_json(&raw);
    let out = itemr.and_then(|item| item::to_json(&item));
    println!("{}", &out.unwrap());
}
