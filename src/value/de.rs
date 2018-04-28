// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

use super::Value;

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
