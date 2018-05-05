// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug, Display};
use url;
use super::Parse;

#[derive(Debug, Fail)]
pub enum UrlError {
    #[fail(display = "Invalid protocol. Url must be either http or https")]
    InvalidProtocol,
    #[fail(display = "Invalid port number")]
    InvalidPort,
    #[fail(display = "Invalid IPv4 address")]
    InvalidIpv4Address,
    #[fail(display = "Invalid IPv6 address")]
    InvalidIpv6Address,
    #[fail(display = "Invalid domain")]
    InvalidDomain,
    #[fail(display = "Relative URL")]
    RelativeUrl,
    #[fail(display = "URLs more than 4 GB are not supported")]
    Overflow,
    #[fail(display = "Unexpected URL")]
    ParseError,
}

/// A Url resource
///
/// ```
/// use sac::value::Parse;
/// use sac::value::url::Url;
/// let url = Url::parse("https://example.org/foo:bar").unwrap();
/// assert_eq!(url.to_string(), "https://example.org/foo:bar".to_string());
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

impl Parse for Url {
    type Atom = Url;
    type Error = UrlError;
    fn parse(s: &str) -> Result<Self::Atom, Self::Error> {
        let u = url::Url::parse(s)?;

        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(Url(u))
        } else {
            Err(UrlError::InvalidProtocol)
        }
    }
}

impl From<url::ParseError> for UrlError {
    fn from(err: url::ParseError) -> UrlError {
        match err {
            url::ParseError::InvalidPort => UrlError::InvalidPort,
            url::ParseError::InvalidIpv4Address => UrlError::InvalidIpv4Address,
            url::ParseError::InvalidIpv6Address => UrlError::InvalidIpv6Address,
            url::ParseError::InvalidDomainCharacter => UrlError::InvalidDomain,
            url::ParseError::RelativeUrlWithoutBase => UrlError::RelativeUrl,
            url::ParseError::RelativeUrlWithCannotBeABaseBase => UrlError::RelativeUrl,
            url::ParseError::Overflow => UrlError::Overflow,
            _ => UrlError::ParseError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let expected = r#"Ok("https://example.org/")"#.to_string();
        let actual = Url::parse("https://example.org");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn fail_with_non_absolute() {
        let expected = r#"Err(RelativeUrl)"#.to_string();
        let actual = Url::parse("foo");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn fail_with_non_http() {
        let expected = r#"Err(InvalidProtocol)"#.to_string();
        let actual = Url::parse("ftp://example.org");

        assert_eq!(format!("{:?}", actual), expected);
    }

}
