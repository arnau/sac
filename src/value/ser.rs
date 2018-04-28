// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use serde::ser::{Serialize, Serializer};

use super::Value;

// TODO: Find a way to uppercase HEX here.
// See https://docs.serde.rs/src/serde_json/ser.rs.html#1395-1415
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Untyped(ref v) => serializer.serialize_str(v),
            _ => unimplemented!(),
            // Value::Unknown => "".to_string(),
            // Value::Inapplicable => "".to_string(),
            // Value::Bool(v) => v.to_string(),
            // Value::String(ref v) => v,
            // Value::Text(ref v) => v,
            // Value::List(ref v) => v.map(ToString).collect(),
            // Value::Integer(ref v) => v.to_string(),
            // Value::DateTime(ref v) => Debug::fmt(v, formatter),
            // Value::Timestamp(ref v) => Debug::fmt(v, formatter),
            // Value::Period(ref v) => Debug::fmt(v, formatter),
            // Value::Point(ref v) => Debug::fmt(v, formatter),
            // Value::Polygon(ref v) => Debug::fmt(v, formatter),
            // Value::Curie(ref v) => Debug::fmt(v, formatter),
            // Value::Hash(ref v) => Debug::fmt(v, formatter),
            // Value::Url(ref v) => Debug::fmt(v, formatter),
        }
    }
}
