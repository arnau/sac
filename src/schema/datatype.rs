// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Display};
use std::str::FromStr;

pub const PRIMITIVES: &'static [&'static str] = &[
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
