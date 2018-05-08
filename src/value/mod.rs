// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug, Display};
use std::str::{FromStr, ParseBoolError};
use std::num::ParseIntError;

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

use self::curie::Curie;
use self::datetime::Datetime;
use self::hash::{Hash, HashError};
use self::integer::Integer;
use self::period::Period;
use self::point::Point;
use self::polygon::Polygon;
use self::text::{Text, TextError};
use self::timestamp::Timestamp;
use self::url::{Url, UrlError};
use kind::Kind;

/// Represents a validation error for item values. Ranges from parsing issues
/// to type checks.
#[derive(Debug, Fail)]
pub enum ValueError {
    #[fail(display = "Invalid value {}", value)]
    InvalidValue { value: String },
    #[fail(display = "Unknown type {}", kind)]
    UnknownType { kind: String },
    #[fail(display = "Invalid url")]
    InvalidUrl(UrlError),
    #[fail(display = "Invalid boolean")]
    InvalidBool(ParseBoolError),
    #[fail(display = "Invalid integer")]
    InvalidInteger(ParseIntError),
    #[fail(display = "Invalid unknown")]
    InvalidUnknown,
    #[fail(display = "Invalid inapplicable")]
    InvalidInapplicable,
    #[fail(display = "Invalid text")]
    InvalidText(TextError),
    #[fail(display = "Invalid hash")]
    InvalidHash(HashError),
}

/// An interface to guarantee values can be checked for correctness.
///
/// TODO: Consider using FromStr instead.
pub trait Parse {
    type Atom;
    type Error;
    fn parse(s: &str) -> Result<Self::Atom, Self::Error>;
}

/// Represents a Blob Value.
///
/// The spec defines the following:
/// * Entry-reference (???). -- Some sort of CURIE.
/// * Fieldname -- This is because the field register needs a restricted string.
///   If we move away from central schemas, there is no need for it.
#[derive(Clone, PartialEq)]
pub enum Value {
    // An untyped value allows constructing an item without a known schema.
    // TODO: Is it better to have another value implementation for this case?
    // Perhaps Value should be a trait?
    Untyped(String),
    // Cardinality n
    // TODO: It shouldn't be possible to have a list of lists of values.
    List(Vec<Value>),

    /// Represents an applicable missing value.
    ///
    /// * In JSON this is encoded as `null`.
    /// * In CSV this is encoded as an empty value.
    /// TODO: What if instead of encoding it, we ignore it entirely at deserialize time?
    Unknown,

    /// Represents an inapplicable value. Note this value is not part of the
    /// spec.
    ///
    /// * In JSON this is encoded as `{"type": "inapplicable"}`.
    /// * In CSV this is encoded as `N/A`.
    Inapplicable,

    /// Represents a boolean. Note this value is not part of the spec.
    ///
    /// * In JSON this is encoded as `true | false`.
    /// * In CSV this is encoded as `true | false`.
    Bool(bool),

    /// Represents a UTF-8 string as defined by RFC7159.
    ///
    /// * In JSON this is encoded as a string.
    /// * In CSV this is encoded as a string.
    String(String),

    /// Represents a UTF-8 string as defined by [http://spec.commonmark.org/].
    /// It is recommended to use the core set of features (i.e. paragraphs,
    /// headers, bold, italica, links and inline code).
    ///
    /// TODO: For security reasons, the Unicode character U+0000 must be replaced with the
    /// REPLACEMENT CHARACTER (U+FFFD).
    Text(Text),

    // An decimal integer.
    // TODO: spec doesn't allow floating point numbers.
    Integer(Integer),
    // TODO: ISO8601 datetime UTC only.
    Datetime(Datetime),
    // TODO: ISO8601 timestamp UTC only. Requires all atoms in date and time.
    // It could (should?) be represented as epoch.
    Timestamp(Timestamp),
    // TODO: ISO8601 Period.
    Period(Period),
    // TODO: GeoJSON.
    Point(Point),
    // TODO: GeoJSON.
    Polygon(Polygon),
    Curie(Curie),
    // Hex with hashing algorithm
    Hash(Hash),
    Url(Url),
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::Curie(ref v) => Debug::fmt(v, formatter),
            Value::Datetime(ref v) => Debug::fmt(v, formatter),
            Value::Hash(ref v) => Debug::fmt(v, formatter),
            Value::Inapplicable => formatter.debug_tuple("Inapplicable").finish(),
            Value::Integer(ref v) => Debug::fmt(v, formatter),
            Value::List(ref v) => formatter.debug_tuple("List").field(v).finish(),
            Value::Period(ref v) => Debug::fmt(v, formatter),
            Value::Point(ref v) => Debug::fmt(v, formatter),
            Value::Polygon(ref v) => Debug::fmt(v, formatter),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Text(ref v) => Debug::fmt(v, formatter),
            Value::Timestamp(ref v) => Debug::fmt(v, formatter),
            Value::Unknown => formatter.debug_tuple("Unknown").finish(),
            Value::Untyped(ref v) => formatter.debug_tuple("Untyped").field(v).finish(),
            Value::Url(ref v) => formatter.debug_tuple("Url").field(v).finish(),
        }
    }
}

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value::Untyped(s.to_owned()))
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(ref v) => Display::fmt(v, formatter),
            Value::Hash(ref v) => Debug::fmt(v, formatter),
            Value::Inapplicable => Display::fmt("N/A", formatter),
            Value::Integer(ref v) => Display::fmt(v, formatter),
            Value::String(ref v) => Display::fmt(v, formatter),
            Value::Text(ref v) => Display::fmt(v, formatter),
            Value::Unknown => Display::fmt("null", formatter),
            Value::Untyped(ref v) => Display::fmt(v, formatter),
            Value::Url(ref v) => Display::fmt(v, formatter),
            _ => unimplemented!(),
            // Value::List(ref v) => v.map(ToString).collect(),
            // Value::Datetime(ref v) => Debug::fmt(v, formatter),
            // Value::Timestamp(ref v) => Debug::fmt(v, formatter),
            // Value::Period(ref v) => Debug::fmt(v, formatter),
            // Value::Point(ref v) => Debug::fmt(v, formatter),
            // Value::Polygon(ref v) => Debug::fmt(v, formatter),
            // Value::Curie(ref v) => Debug::fmt(v, formatter),
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Unknown
    }
}

impl Value {
    pub fn parse(s: &str, kind: Kind) -> Result<Self, ValueError> {
        match kind {
            Kind::Bool => {
                let b = s.parse::<bool>()?;
                Ok(Value::Bool(b))
            }
            // Kind::Curie,
            // Kind::Datetime,
            Kind::Hash => {
                let hash = s.parse::<Hash>()?;
                Ok(Value::Hash(hash))
            }
            Kind::Inapplicable => {
                let s = s.to_lowercase();
                if s == "na" || s == "n/a" {
                    Ok(Value::Inapplicable)
                } else {
                    Err(ValueError::InvalidInapplicable)
                }
            }
            Kind::Integer => {
                let i = s.parse::<i64>()?;
                Ok(Value::Integer(Integer(i)))
            }
            // Kind::List(Box<Kind>),
            // Kind::Period,
            // Kind::Point,
            // Kind::Polygon,
            Kind::String => Ok(Value::String(s.to_owned())),
            Kind::Text => {
                let text = Text::parse(s)?;
                Ok(Value::Text(text))
            }
            // Kind::Timestamp,
            Kind::Unknown => {
                let s = s.to_lowercase();
                if s == "null" {
                    Ok(Value::Unknown)
                } else {
                    Err(ValueError::InvalidUnknown)
                }
            }
            Kind::Untyped => Ok(Value::Untyped(s.to_owned())),
            Kind::Url => {
                let url = Url::parse(s)?;
                Ok(Value::Url(url))
            }
            _ => Ok(Value::Untyped(s.to_owned())),
        }
    }
}

impl From<UrlError> for ValueError {
    fn from(err: UrlError) -> ValueError {
        ValueError::InvalidUrl(err)
    }
}

impl From<ParseBoolError> for ValueError {
    fn from(err: ParseBoolError) -> ValueError {
        ValueError::InvalidBool(err)
    }
}

impl From<ParseIntError> for ValueError {
    fn from(err: ParseIntError) -> ValueError {
        ValueError::InvalidInteger(err)
    }
}

impl From<TextError> for ValueError {
    fn from(err: TextError) -> ValueError {
        ValueError::InvalidText(err)
    }
}

impl From<HashError> for ValueError {
    fn from(err: HashError) -> ValueError {
        ValueError::InvalidHash(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::kind::Kind;

    #[test]
    fn parse_bool() {
        let expected = r#"Ok(Bool(true))"#.to_string();
        let actual = Value::parse("true", Kind::Bool);

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_integer() {
        let expected = r#"Ok(Integer(0))"#.to_string();
        let actual = Value::parse("0", Kind::Integer);

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_url() {
        let expected = r#"Ok(Url("https://example.org/"))"#.to_string();
        let actual = Value::parse("https://example.org", Kind::Url);

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_text() {
        let expected = r#"Ok(Text("foo *bar*"))"#.to_string();
        let actual = Value::parse("foo *bar*", Kind::Text);

        assert_eq!(format!("{:?}", actual), expected);
    }
}
