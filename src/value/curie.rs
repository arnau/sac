// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::Regex;
use std::str::FromStr;
use std::fmt::{self, Debug, Display};
use failure;
use super::Parse;

/// A restricted version of a CURIE defined by the W3C.
///
/// TODO: Might be a good idea to rename this to `cref` (compact reference) or
/// similar.
#[derive(Clone, PartialEq)]
pub struct Curie {
    prefix: Prefix,
    reference: Reference,
}

impl Debug for Curie {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_tuple("Curie")
            .field(&self.prefix)
            .field(&self.reference)
            .finish()
    }
}

impl Display for Curie {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}:{}", self.prefix, self.reference)
    }
}

impl Parse for Curie {
    type Err = failure::Error;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();

        if v.len() < 2 {
            bail!("A curie must be of the form <prefix>:<value>")
        } else {
            let prefix = Prefix::from_str(v[0])?;
            let value = Reference::from_str(v[1])?;

            Ok(Curie::new(prefix, value))
        }
    }
}

impl Curie {
    pub fn new(prefix: Prefix, reference: Reference) -> Self {
        Curie {
            prefix: prefix,
            reference: reference,
        }
    }
}

/// A CURIE prefix is a string conforming to NCName but in Registers it must
/// conform to a valid Register ID: [a-z][a-z0-9-]
///
///
/// TODO: Ensure prefixes and register IDs are compatible.
#[derive(Clone, PartialEq)]
pub struct Prefix(String);

impl Debug for Prefix {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, formatter)
    }
}

impl Display for Prefix {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for Prefix {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self::new(s))
        } else {
            bail!("A prefix must conform `[a-z][a-z0-9-]+`")
        }
    }
}

impl Prefix {
    pub fn new(s: &str) -> Self {
        Prefix(s.into())
    }

    pub fn is_valid(s: &str) -> bool {
        lazy_static! {
        static ref RE: Regex = Regex::new(r#"[a-z][a-z0-9-]+"#).unwrap();
        }

        RE.is_match(s)
    }
}

/// A CURIE reference is a string conforming to irelative-ref in [RFC3987]
///
/// TODO: Consider skipping RFC3987 and conform to https://url.spec.whatwg.org/
#[derive(Clone, PartialEq)]
pub struct Reference(String);

impl Debug for Reference {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, formatter)
    }
}

impl Display for Reference {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for Reference {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self::new(s))
        } else {
            bail!("Invalid CURIE reference")
        }
    }
}

impl Reference {
    pub fn new(s: &str) -> Self {
        Reference(s.into())
    }

    // TODO: Validate conforms to RFC3987
    pub fn is_valid(s: &str) -> bool {
        !s.ends_with(':')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let curie = Curie::parse("foo:bar").unwrap();

        assert_eq!(curie.prefix, Prefix::new("foo"));
        assert_eq!(curie.reference, Reference::new("bar"));
    }

    #[test]
    fn from_str_empty_ref() {
        let curie = Curie::parse("foo:").unwrap();

        assert_eq!(curie.prefix, Prefix::new("foo"));
        assert_eq!(curie.reference, Reference::new(""));
    }

    #[test]
    fn from_str_wrong_syntax() {
        let res = Curie::parse("foo");

        assert!(res.is_err(), "Expected curie to be an error");
    }

    #[test]
    fn from_str_wrong_reference() {
        let res = Curie::parse("foo:bar:");

        assert!(res.is_err(), "Expected curie to be an error");
    }

    mod prefix {
        use super::*;
        #[test]
        fn from_valid_str() {
            assert!(
                Prefix::from_str("foo").is_ok(),
                "Expected prefix to be a valid str"
            );
        }
    }

    mod reference {
        use super::*;
        #[test]
        fn from_valid_str() {
            assert!(
                Reference::from_str("bar").is_ok(),
                "Expected reference to be a valid str"
            );
        }

        #[test]
        fn valid_with_slash() {
            assert!(
                Reference::from_str("bar/qux").is_ok(),
                "Expected reference to be a valid str"
            );
        }

        #[test]
        fn valid_with_colon() {
            assert!(
                Reference::from_str("bar:qux").is_ok(),
                "Expected reference to be a valid str"
            );
        }

    }
}
