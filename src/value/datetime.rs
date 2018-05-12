// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::RegexSet;
use std::fmt::{self, Debug, Display};
use super::Parse;

#[derive(Debug, Fail)]
pub enum DatetimeError {
    #[fail(display = "Invalid ISO8601 datetime.")]
    ParseError,
}

/// ISO8601 Date time
#[derive(Clone, PartialEq)]
pub struct Datetime(String);

impl Debug for Datetime {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Datetime").field(&self.0).finish()
    }
}

impl Display for Datetime {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

/// TODO: Only pattern checks are performed. An out of range date will be
/// accepted as valid.
impl Parse for Datetime {
    type Err = DatetimeError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: RegexSet = RegexSet::new(&[
                r"^\d{4}$",
                r"^\d{4}-\d{2}$",
                r"^\d{4}-\d{2}-\d{2}$",
                r"^\d{4}-\d{2}-\d{2}T\d{2}Z$",
                r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}Z$",
                r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$",
            ]).unwrap();
        }

        if RE.is_match(s) {
            Ok(Datetime(s.to_owned()))
        } else {
            Err(DatetimeError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_year() {
        let expected = r#"Ok(Datetime("2018"))"#.to_string();
        let actual = Datetime::parse("2018");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_yearmonth() {
        let expected = r#"Ok(Datetime("2018-01"))"#.to_string();
        let actual = Datetime::parse("2018-01");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_date() {
        let expected = r#"Ok(Datetime("2018-01-01"))"#.to_string();
        let actual = Datetime::parse("2018-01-01");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datehour() {
        let expected = r#"Ok(Datetime("2018-01-01T10Z"))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datehourminute() {
        let expected = r#"Ok(Datetime("2018-01-01T10:11Z"))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10:11Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datetime() {
        let expected = r#"Ok(Datetime("2018-01-01T10:11:12Z"))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10:11:12Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

}
