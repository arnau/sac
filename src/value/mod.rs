// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::str::FromStr;

pub mod curie;
pub mod datetime;
pub mod hash;
pub mod integer;
pub mod period;
pub mod point;
pub mod polygon;
pub mod text;
pub mod timestamp;
pub mod url;

pub mod ser;
pub mod de;

/// Represents a validation error for item values. Ranges from parsing issues
/// to type checks.
#[derive(Debug, Fail)]
pub enum ValueError {
    #[fail(display = "invalid value {}", value)]
    InvalidValue { value: String },
    #[fail(display = "unknown type {}", kind)]
    UnknownType { kind: String },
}

/// Represents a typed Item Value.
///
/// TODO: Add all defined datatypes. One branch should be an Untyped string so
/// schemaless processes can be done without the overhead of a schema.
#[derive(Debug, Clone)]
pub struct Value(String);

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value(s.to_owned()))
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
