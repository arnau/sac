// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

//! Blob of data (item resource in the spec)

use failure::Error;
use serde_json;
use std::str::FromStr;
use std::collections::BTreeMap;
use regex::{Captures, Regex};
use digest;

// use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{Serialize, SerializeMap, Serializer};

use field::Fieldname;
use value::Value;

type Nub = BTreeMap<Fieldname, Value>;

// https://docs.rs/serde-transcode/1.0.0/serde_transcode/

/// Serialises a Blob into the canonical JSON string. This is:
/// * Must be a valid JSON object according to RFC7159.
/// * All insignificant whitespace according to RFC7159 MUST be removed.
/// * Object keys must be a valid field name (alphabet of lower-case letters
///   and hyphens ([a-z-]).
/// * MUST be sorted into lexicographical order.
/// * Unicode sequences \uXXXX MUST be in upper-case.
/// * The forward slash or solidus (/) MUST be unescaped.
/// * Non-control characters (i.e. out of the range \u0000 .. \u001F) MUST be
///   unescaped.
///
/// # Examples
///
/// ```json
/// {"foo": "abc"}               # => Ok: {"foo":"abc"}
/// {"foo": "abc", "bar": "xyz"} # => Ok: {"bar":"xyz","foo":"abc"}
/// {"Foo": "abc"}               # => Error: invalid field name
/// ```
pub fn to_json(blob: &Blob) -> Result<String, Error> {
    let s = serde_json::to_string(&blob).map(|x| uppercase_hex(&x))?;

    Ok(s)
}

/// Deserialises a valid JSON object into a Blob. Note that the JSON object
/// must have valid keys as restricted by the canonicalisation algorithm.
pub fn from_json(s: &str) -> Result<Blob, Error> {
    Blob::from_str(s)
}

/// Represents a Blob resource.
///
/// # Examples
///
/// ```
/// let raw = r#"{"foo": "abc", "bar": "xyz"}"#;
/// let blob = sac::blob::from_json(raw).unwrap();
/// assert_eq!(blob.hash(), "5dd4fe3b0de91882dae86b223ca531b5c8f2335d9ee3fd0ab18dfdc2871d0c61");
/// ```
#[derive(Debug, Deserialize, Default)]
pub struct Blob(Nub);
impl Blob {
    pub fn new() -> Self {
        Blob(BTreeMap::new())
    }

    // TODO: Add key, value validation here. Result<(), Error>
    pub fn insert(&mut self, k: Fieldname, v: Value) {
        self.0.insert(k, v);
    }

    pub fn nub(&self) -> Nub {
        self.0.clone()
    }

    pub fn hash(&self) -> String {
        to_json(self)
            .map(|s| digest::to_hex(digest::digest(&s).as_ref()))
            .unwrap()
    }

    // TODO: Generalise algorithm
    pub fn id(&self) -> String {
        format!("sha-256:{}", self.hash())
    }
}

// TODO: https://doc.rust-lang.org/std/hash/trait.Hash.html
// pub trait Hash {
//     fn hash<H>(&self, state: &mut H)
//     where
//         H: Hasher;

//     fn hash_slice<H>(data: &[Self], state: &mut H)
//     where
//         H: Hasher,
//     { ... }
// }

impl FromStr for Blob {
    type Err = Error;
    fn from_str(s: &str) -> Result<Blob, Self::Err> {
        let blob = serde_json::from_str::<Blob>(s)?;

        Ok(blob)
    }
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

// TODO: Review if it's better to handle deserialisation by hand
// struct CanonicalVisitor;

// impl<'de> Visitor<'de> for CanonicalVisitor {
//     type Value = Canonical;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("Expecting an blob-like structure.")
//     }

//     fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
//         where M: MapAccess<'de>
//     {
//         let mut map = Canonical::new();

//         while let Some((key, value)) = access.next_entry()? {
//             map.insert(key, value);
//         }

//         Ok(map)
//     }
// }

// impl<'de> Deserialize<'de> for Canonical {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: Deserializer<'de>
//     {
//         deserializer.deserialize_map(CanonicalVisitor)
//     }
// }

fn uppercase_hex(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\\u([a-f0-9]{4})"#).unwrap();
    }

    RE.replace_all(s, |c: &Captures| format!("\\u{}", &c[1].to_uppercase()))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_non_hex_case() {
        let input = "abc";
        let expected = "abc".to_string();
        assert_eq!(uppercase_hex(input), expected);
    }

    #[test]
    fn changes_hex_case() {
        assert_eq!(uppercase_hex("abc\\u001f"), "abc\\u001F".to_string());
        assert_eq!(uppercase_hex("abc\\u001F"), "abc\\u001F".to_string());
        assert_eq!(uppercase_hex("abc\\ucafe"), "abc\\uCAFE".to_string());
    }
}
