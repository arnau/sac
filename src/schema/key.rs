// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;

use super::datatype::{Datatype, Primitive};

/// Primary key (or field)
///
/// Similar to a Field but datatype is imposed.
#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    id: String,
    datatype: Datatype,
    label: Option<String>,
    description: Option<String>,
}
impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Attr {
            Id,
            Label,
            Description,
        }

        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expecting a Key struct.")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut id = None;
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

                Ok(Key::new(
                    id.ok_or(de::Error::missing_field("id"))?,
                    label,
                    description,
                ))
            }
        }

        const ATTRS: &'static [&'static str] = &["id", "label", "description"];
        deserializer.deserialize_struct("Duration", ATTRS, KeyVisitor)
    }
}

impl Key {
    pub fn new(id: &str, label: Option<&str>, description: Option<&str>) -> Self {
        Key {
            id: id.to_string(),
            datatype: Datatype::One(Primitive::String),
            label: label.map(|s| s.into()),
            description: description.map(|s| s.into()),
        }
    }
}
