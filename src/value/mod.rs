// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug};
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

use self::curie::Curie;
use self::datetime::Datetime;
use self::hash::Hash;
use self::integer::Integer;
use self::period::Period;
use self::point::Point;
use self::polygon::Polygon;
use self::text::Text;
use self::timestamp::Timestamp;
use self::url::Url;

/// Represents a validation error for item values. Ranges from parsing issues
/// to type checks.
#[derive(Debug, Fail)]
pub enum ValueError {
    #[fail(display = "invalid value {}", value)]
    InvalidValue { value: String },
    #[fail(display = "unknown type {}", kind)]
    UnknownType { kind: String },
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
            Value::Untyped(ref v) => formatter.debug_tuple("Untyped").field(v).finish(),
            Value::List(ref v) => formatter.debug_tuple("List").field(v).finish(),
            Value::Unknown => formatter.debug_tuple("Unknown").finish(),
            Value::Inapplicable => formatter.debug_tuple("Inapplicable").finish(),
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Text(ref v) => Debug::fmt(v, formatter),
            Value::Integer(ref v) => Debug::fmt(v, formatter),
            Value::Datetime(ref v) => Debug::fmt(v, formatter),
            Value::Timestamp(ref v) => Debug::fmt(v, formatter),
            Value::Period(ref v) => Debug::fmt(v, formatter),
            Value::Point(ref v) => Debug::fmt(v, formatter),
            Value::Polygon(ref v) => Debug::fmt(v, formatter),
            Value::Curie(ref v) => Debug::fmt(v, formatter),
            Value::Hash(ref v) => Debug::fmt(v, formatter),
            Value::Url(ref v) => Debug::fmt(v, formatter),
        }
    }
}

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value::Untyped(s.to_owned()))
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match *self {
            Value::Untyped(ref v) => v.to_owned(),
            _ => "unimplemented".to_owned(),
            // Value::Unknown => "".to_string(),
            // Value::Inapplicable => "".to_string(),
            // Value::Bool(v) => v.to_string(),
            // Value::String(ref v) => v,
            // Value::Text(ref v) => v,
            // Value::List(ref v) => v.map(ToString).collect(),
            // Value::Integer(ref v) => v.to_string(),
            // Value::Datetime(ref v) => Debug::fmt(v, formatter),
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

impl Default for Value {
    fn default() -> Value {
        Value::Unknown
    }
}
