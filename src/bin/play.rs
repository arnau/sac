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

use sac::datatypes::{Integer, Value};

fn main() {
    let u = Value::Untyped("foo".into());
    println!("{:?}", u);

    println!("{:?}", Value::String("bar".into()));
    println!("{:?}", Value::Integer(Integer(1)));
    println!("{:?}", Value::Point("a".into(), "b".into()));
}
