// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;

use super::datatype::{Datatype, Primitive, PRIMITIVES};

/// Representa a schema field to validate in a blob of data.
///
/// ## Example
///
/// ```
/// use sac::schema::*;
///
/// // start-date:datetime
/// let start_date = Field::new(
///      "start-date".to_string(),
///      Datatype::One(Primitive::Datetime),
///      None,
///      None,
/// );
/// // citizen-names:[string]
/// let citizen_names = Field::new(
///     "citizen-names".to_string(),
///     Datatype::Many(Primitive::String),
///     Some("Citizen names".to_string()),
///     Some("The names of a country's citizen.".to_string()),
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    id: String,
    datatype: Datatype,
    label: Option<String>,
    description: Option<String>,
}

impl Field {
    pub fn new(
        id: String,
        datatype: Datatype,
        label: Option<String>,
        description: Option<String>,
    ) -> Self {
        Field {
            id,
            datatype,
            label,
            description,
        }
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Attr {
            Id,
            Type,
            Cardinality,
            Label,
            Description,
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

                while let Some(key) = map.next_key()? {
                    match key {
                        Attr::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Attr::Type => {
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
                        Attr::Cardinality => {
                            if cardinality.is_some() {
                                return Err(de::Error::duplicate_field("cardinality"));
                            }
                            cardinality = Some(map.next_value()?);
                        }
                        Attr::Label => {
                            if label.is_some() {
                                return Err(de::Error::duplicate_field("label"));
                            }
                            label = Some(map.next_value()?);
                        }
                        Attr::Description => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
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

                Ok(Field::new(
                    id.ok_or(de::Error::missing_field("id"))?,
                    datatype,
                    label,
                    description,
                ))
            }
        }

        const ATTRS: &'static [&'static str] =
            &["id", "type", "cardinality", "label", "description"];
        deserializer.deserialize_struct("Field", ATTRS, FieldVisitor)
    }
}
