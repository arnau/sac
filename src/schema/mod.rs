// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use toml;

mod datatype;
mod field;
mod key;

pub use self::datatype::{Datatype, Primitive, PRIMITIVES};
pub use self::field::Field;
pub use self::key::Key;

#[derive(Debug, Fail)]
pub enum SchemaError {
    #[fail(display = "Invalid schema.")]
    ParseError(#[cause] toml::de::Error),
    #[fail(display = "Invalid schema.")]
    IoError(#[cause] io::Error),
    #[fail(display = "Missing primary key.")]
    MissingPrimaryKey,
    #[fail(display = "A schema needs at least one field.")]
    MissingFields,
}

impl From<io::Error> for SchemaError {
    fn from(err: io::Error) -> SchemaError {
        SchemaError::IoError(err)
    }
}

impl From<toml::de::Error> for SchemaError {
    fn from(err: toml::de::Error) -> SchemaError {
        SchemaError::ParseError(err)
    }
}

/// Plan
///
/// ```
/// use sac::schema::*;
///
/// let key = Key::new("id", Some("ID"), None);
/// let start_date = Field::new(
///     "start-date".to_string(),
///     Datatype::One(Primitive::Datetime),
///     None,
///     None,
/// );
/// let planned_schema: Result<Schema, SchemaError> = Plan::new("country")
///     .with_primary_key(key)
///     .with_label("Country")
///     .with_description("Lorem ipsum")
///     .with_custodian("Bob <b@b.b>")
///     .with_field(start_date)
///     .validate();
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Plan {
    id: String,
    #[serde(rename = "primary-key")]
    primary_key: Option<Key>,
    label: Option<String>,
    description: Option<String>,
    custodian: Option<String>,
    fields: Vec<Field>,
}

impl Plan {
    pub fn new(id: &str) -> Self {
        Plan {
            id: id.to_string(),
            primary_key: None,
            label: None,
            description: None,
            custodian: None,
            fields: Vec::new(),
        }
    }

    pub fn with_label<'a>(&'a mut self, label: &str) -> &'a mut Plan {
        self.label = Some(label.to_string());
        self
    }

    pub fn with_description<'a>(&'a mut self, description: &str) -> &'a mut Plan {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_custodian<'a>(&'a mut self, custodian: &str) -> &'a mut Plan {
        self.custodian = Some(custodian.to_string());
        self
    }

    pub fn with_fields<'a>(&'a mut self, fields: Vec<Field>) -> &'a mut Plan {
        self.fields = fields;
        self
    }

    pub fn with_field<'a>(&'a mut self, field: Field) -> &'a mut Plan {
        self.fields.push(field);
        self
    }

    pub fn with_primary_key<'a>(&'a mut self, key: Key) -> &'a mut Plan {
        self.primary_key = Some(key);
        self
    }

    pub fn validate(&self) -> Result<Schema, SchemaError> {
        Ok(Schema {
            id: self.id.clone(),
            primary_key: self.primary_key
                .clone()
                .ok_or(SchemaError::MissingPrimaryKey)?,
            label: self.label.clone(),
            description: self.description.clone(),
            custodian: self.custodian.clone(),
            fields: self.fields.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Schema {
    id: String,
    #[serde(rename = "primary-key")]
    primary_key: Key,
    label: Option<String>,
    description: Option<String>,
    custodian: Option<String>,
    fields: Vec<Field>,
}

impl Schema {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Schema, SchemaError> {
        let mut raw = String::new();

        File::open(path).and_then(|mut f| f.read_to_string(&mut raw))?;

        Schema::from_toml(&raw)
    }

    pub fn from_toml(raw: &str) -> Result<Schema, SchemaError> {
        toml::from_str::<Schema>(&raw).map_err(|err| SchemaError::ParseError(err))
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn primary_key(&self) -> &Key {
        &self.primary_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_schema_from_toml() {
        let toml = r#"
            id = "DB68BB24-8FCB-4172-A74A-84C6225CCABF"
            label = "Foo"
            custodian = "Me"
            primary-key = { id = "id", label = "ID" }

            [[fields]]
            id = "name"
            type = "string"
            cardinality = "1"
            label = "Name"
        "#;

        let actual = Schema::from_toml(toml);
        let expected = Schema {
            id: "DB68BB24-8FCB-4172-A74A-84C6225CCABF".to_string(),
            description: None,
            label: Some("Foo".to_string()),
            custodian: Some("Me".to_string()),
            primary_key: Key::new("id", Some("ID"), None),
            fields: vec![Field::new(
                "name".to_string(),
                Datatype::One(Primitive::String),
                Some("Name".to_string()),
                None,
            )],
        };

        assert_eq!(actual.ok(), Some(expected));
    }
}
