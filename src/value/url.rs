// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug, Display};
use failure;
use url;

/// A Url resource
///
/// ```
/// let raw = "https://example.org/foo:bar";
/// let u = sac::value::url::Url::parse(raw).unwrap();
/// assert_eq!(u.to_string(), raw.to_string());
/// ```
#[derive(Clone, PartialEq)]
pub struct Url(url::Url);

impl Debug for Url {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.0, formatter)
    }
}

impl Display for Url {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl Url {
    pub fn parse(s: &str) -> Result<Self, failure::Error> {
        let u = url::Url::parse(s)?;

        Ok(Url(u))
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
