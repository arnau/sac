use std::str::FromStr;
use regex::Regex;
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor};


#[derive(Debug, Fail)]
pub enum FieldError {
    #[fail(display = "invalid field name {}", name)]
    InvalidFieldname { name: String },
    #[fail(display = "unknown field {}", name)]
    UnknownField { name: String },
}


// The spec requires a fieldname to be [a-z-]. The constructor `from_str`
// ensures this constraint is met.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Fieldname(String);


impl FromStr for Fieldname {
    type Err = FieldError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("([^a-z-])").unwrap();
        }

        if RE.is_match(&key) {
            Err(FieldError::InvalidFieldname { name: key.to_owned() })
        } else {
            Ok(Fieldname(key.to_owned()))
        }
    }
}

impl ToString for Fieldname {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

struct FieldnameVisitor;

impl<'de> Visitor<'de> for FieldnameVisitor {
    type Value = Fieldname;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting a valid fild name.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Fieldname::from_str(value).map_err(de::Error::custom)
    }
}


impl<'de> Deserialize<'de> for Fieldname {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(FieldnameVisitor)
    }
}
