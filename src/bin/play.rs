extern crate sac;
extern crate toml;

use sac::schema::*;

fn main() {
    let raw = "[integer]";
    let i = raw.parse::<Datatype>();

    println!("{:?}", i);
    println!("{}\n\n", i.unwrap());

    // TOML

    let path = "/Users/arnau/kitchen/sac/examples/country.toml";
    let schema = Schema::open(&path).unwrap();

    // println!("{:?}", schema);

    for field in schema.fields() {
        println!("{:?}", field);
    }

    println!("\n\n{:?}", schema.primary_key());

    // Schema plan

    let key = Key::new("id", Some("ID"), None);

    let planned_schema: Result<Schema, SchemaError> = Plan::new("country")
        .with_label("Country")
        .with_description("Lorem ipsum")
        .with_custodian("Bob <b@b.b>")
        .with_primary_key(key)
        .validate();

    println!("\n\n{:?}", planned_schema);
}
