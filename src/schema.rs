use std::fmt::{self, Display};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use toml;

// TODO: Would it be better to split responsibilities between deserializer from
// TOML, JSON, etc AND internal usage?

const PRIMITIVES: &'static [&'static str] = &[
    "bool",
    "curie",
    "datetime",
    "hash",
    "inapplicable",
    "integer",
    "period",
    "point",
    "polygon",
    "string",
    "text",
    "timestamp",
    "unknown",
    "untyped",
    "url",
];

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Primitive {
    Bool,
    Curie,
    Datetime,
    Hash,
    Inapplicable,
    Integer,
    Period,
    Point,
    Polygon,
    String,
    Text,
    Timestamp,
    Unknown,
    Untyped,
    Url,
}

impl Display for Primitive {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Primitive::Bool => Display::fmt("bool", formatter),
            Primitive::Curie => Display::fmt("curie", formatter),
            Primitive::Datetime => Display::fmt("datetime", formatter),
            Primitive::Hash => Display::fmt("hash", formatter),
            Primitive::Inapplicable => Display::fmt("inapplicable", formatter),
            Primitive::Integer => Display::fmt("integer", formatter),
            Primitive::Period => Display::fmt("period", formatter),
            Primitive::Point => Display::fmt("point", formatter),
            Primitive::Polygon => Display::fmt("polygon", formatter),
            Primitive::String => Display::fmt("string", formatter),
            Primitive::Text => Display::fmt("text", formatter),
            Primitive::Timestamp => Display::fmt("timestamp", formatter),
            Primitive::Unknown => Display::fmt("unknown", formatter),
            Primitive::Untyped => Display::fmt("untyped", formatter),
            Primitive::Url => Display::fmt("url", formatter),
        }
    }
}

impl FromStr for Primitive {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bool" => Ok(Primitive::Bool),
            "curie" => Ok(Primitive::Curie),
            "datetime" => Ok(Primitive::Datetime),
            "hash" => Ok(Primitive::Hash),
            "inapplicable" => Ok(Primitive::Inapplicable),
            "integer" => Ok(Primitive::Integer),
            "period" => Ok(Primitive::Period),
            "point" => Ok(Primitive::Point),
            "polygon" => Ok(Primitive::Polygon),
            "string" => Ok(Primitive::String),
            "text" => Ok(Primitive::Text),
            "timestamp" => Ok(Primitive::Timestamp),
            "unknown" => Ok(Primitive::Unknown),
            "untyped" => Ok(Primitive::Untyped),
            "url" => Ok(Primitive::Url),
            _ => Err("Unexpected primitive type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Datatype {
    One(Primitive),
    Many(Primitive),
}

impl Display for Datatype {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Datatype::One(ref primitive) => Display::fmt(primitive, formatter),
            Datatype::Many(ref primitive) => Display::fmt(&format!("[{}]", primitive), formatter),
        }
    }
}

impl FromStr for Datatype {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') && s.ends_with(']') {
            s[1..(s.len() - 1)]
                .parse::<Primitive>()
                .map(|p| Datatype::Many(p))
        } else {
            s.parse::<Primitive>().map(|p| Datatype::One(p))
        }
    }
}

/// ## Example
///
/// ```
/// use sac::schema::*;
/// // country:string:ur
/// let id = Field {
///      id: "country".to_string(),
///      datatype: Datatype::One(Primitive::String),
///      label: Some("ID".to_string()),
///      description: None,
///      unique: true,
///      required: true,
/// };
/// // start-date:datetime
/// let start_date = Field {
///      id: "start-date".to_string(),
///      datatype: Datatype::One(Primitive::Datetime),
///      label: Some("Start date".to_string()),
///      description: None,
///      unique: false,
///      required: false,
/// };
/// // citizen-names:[string]
/// let citizen_names = Field {
///      id: "citizen-names".to_string(),
///      datatype: Datatype::Many(Primitive::String),
///      label: Some("Citizen names".to_string()),
///      description: Some("The names of a country's citizen.".to_string()),
///      unique: false,
///      required: false,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub id: String,
    pub datatype: Datatype,
    pub label: Option<String>,
    pub description: Option<String>,
    pub unique: bool,
    pub required: bool,
}

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Key {
            Id,
            Type,
            Cardinality,
            Label,
            Description,
            Unique,
            Required,
        }

        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expecting a Field struct.")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut id = None;
                let mut primitive = None;
                let mut cardinality = None;
                let mut label = None;
                let mut description = None;
                let mut unique = None;
                let mut required = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Key::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Key::Type => {
                            if primitive.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let raw: String = map.next_value()?;
                            let value = raw.parse::<Primitive>();

                            if value.is_err() {
                                return Err(de::Error::unknown_variant("type", PRIMITIVES));
                            };
                            primitive = value.ok();
                        }
                        Key::Cardinality => {
                            if cardinality.is_some() {
                                return Err(de::Error::duplicate_field("cardinality"));
                            }
                            cardinality = Some(map.next_value()?);
                        }
                        Key::Label => {
                            if label.is_some() {
                                return Err(de::Error::duplicate_field("label"));
                            }
                            label = Some(map.next_value()?);
                        }
                        Key::Description => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        Key::Unique => {
                            if unique.is_some() {
                                return Err(de::Error::duplicate_field("unique"));
                            }
                            unique = Some(map.next_value()?);
                        }
                        Key::Required => {
                            if required.is_some() {
                                return Err(de::Error::duplicate_field("required"));
                            }
                            required = Some(map.next_value()?);
                        }
                    }
                }

                if primitive.is_none() {
                    return Err(de::Error::missing_field("type"));
                }

                let datatype = match cardinality {
                    Some("1") => Datatype::One(primitive.unwrap()),
                    Some("n") => Datatype::Many(primitive.unwrap()),
                    Some(_) => return Err(de::Error::unknown_variant("cardinality", &["1", "n"])),
                    None => return Err(de::Error::missing_field("cardinality")),
                };

                Ok(Field {
                    id: id.ok_or(de::Error::missing_field("id"))?,
                    datatype: datatype,
                    label: label,
                    description: description,
                    unique: unique.unwrap_or(false),
                    required: required.unwrap_or(false),
                })
            }
        }

        // deserializer.deserialize_map(FieldVisitor)
        const KEYS: &'static [&'static str] = &[
            "id",
            "type",
            "cardinality",
            "label",
            "description",
            "unique",
            "required",
        ];
        deserializer.deserialize_struct("Duration", KEYS, FieldVisitor)
    }
}

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

/// Schema
///
/// ```text
/// // TODO:
/// use sac::schema::*;
/// // country:string:ur
/// let field = Field {
///      id: "country".to_string(),
///      datatype: Datatype::One(Primitive::String),
///      label: Some("ID".to_string()),
///      description: None,
///      unique: true,
///      required: true,
/// };
///
/// let schema: Result<Schema, SchemaError> =
///     // alt: Plan::new("country", Datatype::One(Primitive::String))
///     Plan::new()
///         .with_id("country")
///         .with_datatype(Datatype::One(Primitive::String))
///         .with_label("Country")
///         .with_description("Lorem ipsum")
///         .with_custodian("Bob <b@b.b>")
///         .with_fields(vec![field])
///         .with_primary_key("country")
///         .schema(); //-- checks primary key exists.
///
/// let schema_2 = Schema::new(
///     "country",                          // id
///     Datatype::One(Primitive::String),   // datatype
///     Some("Country"),                    // label
///     Some("Lorem ipsum"),                // description
///     Some("Bob <b@b.b>"),                // custodian
///     vec![field],                        // fields
///     "country");                         // primary key. checks that it exists.
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Plan {
    id: String,
    #[serde(rename = "primary-key")]
    primary_key: Option<String>,
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

    pub fn with_primary_key<'a>(&'a mut self, pk: &str) -> &'a mut Plan {
        self.primary_key = Some(pk.to_string());
        self
    }

    pub fn validate(&self) -> Result<Schema, SchemaError> {
        if self.fields.len() == 0 {
            return Err(SchemaError::MissingFields);
        }

        let sch = Schema {
            id: self.id.clone(),
            primary_key: self.primary_key
                .clone()
                .ok_or(SchemaError::MissingPrimaryKey)?,
            label: self.label.clone(),
            description: self.description.clone(),
            custodian: self.custodian.clone(),
            fields: self.fields.clone(),
        };

        let primary_field = self.fields.iter().find(|&x| x.id == sch.primary_key);

        if primary_field.is_none() {
            return Err(SchemaError::MissingPrimaryKey);
        }

        Ok(sch)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Schema {
    id: String,
    #[serde(rename = "primary-key")]
    primary_key: String,
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

    // TODO: Primary key must exist in fields. What's the best place to enforce
    // this? At least Deserialiser should guarantee this.
    pub fn primary_field(&self) -> &Field {
        &self.fields
            .iter()
            .find(|&x| x.id == self.primary_key)
            .unwrap()
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
            primary-key = "id"
            [[fields]]
            id = "id"
            type = "string"
            cardinality = "1"
            label = "ID"
        "#;

        let actual = Schema::from_toml(toml).ok();
        let expected = Schema {
            id: "DB68BB24-8FCB-4172-A74A-84C6225CCABF".to_string(),
            description: None,
            label: Some("Foo".to_string()),
            custodian: Some("Me".to_string()),
            primary_key: "id".to_string(),
            fields: vec![Field {
                id: "id".to_string(),
                datatype: Datatype::One(Primitive::String),
                label: Some("ID".to_string()),
                description: None,
                required: false,
                unique: false,
            }],
        };

        assert_eq!(actual, Some(expected));
    }
}
