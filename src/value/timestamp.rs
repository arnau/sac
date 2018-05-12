// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::Regex;
use std::fmt::{self, Debug, Display};
use super::Parse;

#[derive(Debug, Fail)]
pub enum TimestampError {
    #[fail(display = "Invalid RFC3339 timestamp.")]
    ParseError,
}

/// Timestamp as defined by [RFC3339](https://openregister.github.io/specification/#biblio-rfc3339)
/// constrained to UTC expressed as "Z".
///
/// Note that "T" and "Z" must be capitalised.
#[derive(Clone, PartialEq)]
pub struct Timestamp(String);

impl Debug for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Timestamp").field(&self.0).finish()
    }
}

impl Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

/// TODO: Only pattern checks are performed. An out of range timestamp will be
/// accepted as a valid timestamp.
impl Parse for Timestamp {
    type Err = TimestampError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$"#).unwrap();
        }

        if RE.is_match(s) {
            Ok(Timestamp(s.to_owned()))
        } else {
            Err(TimestampError::ParseError)
        }
    }
}
