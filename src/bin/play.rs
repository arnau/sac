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

    println!("\n\n{:?}", schema.primary_field());

    // Schema plan

    let field = Field {
        id: "country".to_string(),
        datatype: Datatype::One(Primitive::String),
        label: Some("ID".to_string()),
        description: None,
        unique: true,
        required: true,
    };

    let planned_schema: Result<Schema, SchemaError> = Plan::new("country")
        .with_label("Country")
        .with_description("Lorem ipsum")
        .with_custodian("Bob <b@b.b>")
        .with_fields(vec![field])
        .with_primary_key("country")
        .validate();

    println!("\n\n{:?}", planned_schema);
}
