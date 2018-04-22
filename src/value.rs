use serde::ser::{Serialize, Serializer};
use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

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

// TODO: Find a way to uppercase HEX here.
// See https://docs.serde.rs/src/serde_json/ser.rs.html#1395-1415
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting a valid value.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Value::from_str(value).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn basic_string_value() {
        let input = r#""abc""#;
        let expected = r#"Ok(Value("abc"))"#.to_string();
        let res = serde_json::from_str::<Value>(input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn default_escapes_string_value() {
        let input = r#""\b\t\n\r\f\u0000\/""#;
        let expected = r#"Ok(Value("\u{8}\t\n\r\u{c}\u{0}/"))"#.to_string();
        let res = serde_json::from_str::<Value>(input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn unicode_escapes_string_value() {
        let input = r#""❤\u2764""#;
        let expected = r#"Ok(Value("❤❤"))"#.to_string();
        let res = serde_json::from_str::<Value>(input);

        assert_eq!(format!("{:?}", res), expected);
    }
}
