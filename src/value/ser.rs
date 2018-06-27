// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;

use super::Value;

// TODO: Find a way to uppercase HEX here.
// See https://docs.serde.rs/src/serde_json/ser.rs.html#1395-1415
//
// TODO: Decouple List from primitive values the same way Datatype and Primitive
// do
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Value::Untyped(ref v) => serializer.serialize_str(v),
            Value::Unknown => serializer.serialize_unit(),
            Value::Inapplicable => {
                use serde::ser::SerializeMap;
                let m: HashMap<&str, &str> = [("type", "inapplicable")].iter().cloned().collect();
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Bool(v) => serializer.serialize_bool(v),
            Value::Curie(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Datetime(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Hash(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Integer(ref v) => serializer.serialize_i64(v.0),
            Value::Period(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Point(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Polygon(ref v) => serializer.serialize_str(&v.to_string()),
            Value::String(ref v) => serializer.serialize_str(v),
            Value::Text(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Timestamp(ref v) => serializer.serialize_str(&v.to_string()),
            Value::Url(ref v) => serializer.serialize_str(&v.to_string()),

            Value::List(ref xs) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(xs.len()))?;
                for e in xs {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_untyped() {
        let input = Value::Untyped("abc".into());
        let expected = r#"Ok("\"abc\"")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_unknown() {
        let input = Value::Unknown;
        let expected = r#"Ok("null")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_inapplicable() {
        let input = Value::Inapplicable;
        let expected = r#"Ok("{\"type\":\"inapplicable\"}")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_bool_true() {
        let input = Value::Bool(true);
        let expected = r#"Ok("true")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_bool_false() {
        let input = Value::Bool(false);
        let expected = r#"Ok("false")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_string() {
        let input = Value::String("xyz".into());
        let expected = r#"Ok("\"xyz\"")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_integer() {
        use value::integer::Integer;
        let input = Value::Integer(Integer(0));
        let expected = r#"Ok("0")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_integer_list() {
        use value::integer::Integer;
        let list = vec![Value::Integer(Integer(0)), Value::Integer(Integer(1))];
        let input = Value::List(list);
        let expected = r#"Ok("[0,1]")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_string_list() {
        let list = vec![Value::String("a".into()), Value::String("b".into())];
        let input = Value::List(list);
        let expected = r#"Ok("[\"a\",\"b\"]")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

    #[test]
    fn serialize_curie() {
        use value::curie::{Curie, Prefix, Reference};
        let input = Value::Curie(Curie::new(Prefix::new("foo"), Reference::new("bar")));
        let expected = r#"Ok("\"foo:bar\"")"#.to_string();
        let res = serde_json::to_string(&input);

        assert_eq!(format!("{:?}", res), expected);
    }

}
