use std::fmt::{self, Display};
use std::str::FromStr;

// TODO: Is "kind" better than "datatype"?
#[derive(Debug, Clone)]
pub enum Kind {
    Bool,
    Curie,
    Datetime,
    Hash,
    Inapplicable,
    Integer,
    List(Box<Kind>),
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

impl Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Bool,
            // Curie,
            // Datetime,
            // Hash,
            // Inapplicable,
            // Integer,
            // List(Box<Kind>),
            // Period,
            // Point,
            // Polygon,
            // String,
            // Text,
            // Timestamp,
            // Unknown,
            // Untyped,
            // Url,
            ref x => Display::fmt(&format!("{:?}", x), formatter)
            // Value::Inapplicable => Display::fmt("N/A", formatter),
            // Value::Untyped(ref v) => Display::fmt(v, formatter),
        }
    }
}

impl FromStr for Kind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bool" => Ok(Kind::Bool),
            "curie" => Ok(Kind::Curie),
            "datetime" => Ok(Kind::Datetime),
            "hash" => Ok(Kind::Hash),
            "inapplicable" => Ok(Kind::Inapplicable),
            "integer" => Ok(Kind::Integer),
            "period" => Ok(Kind::Period),
            "point" => Ok(Kind::Point),
            "polygon" => Ok(Kind::Polygon),
            "string" => Ok(Kind::String),
            "text" => Ok(Kind::Text),
            "timestamp" => Ok(Kind::Timestamp),
            "unknown" => Ok(Kind::Unknown),
            "untyped" => Ok(Kind::Untyped),
            "url" => Ok(Kind::Url),
            _ => unreachable!(),
        }
    }
}
