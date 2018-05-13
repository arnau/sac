// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::{Regex, RegexSet};
use std::fmt::{self, Debug, Display};
use super::Parse;
use super::datetime::{Datetime, DatetimeError};

#[derive(Debug, Fail)]
pub enum PeriodError {
    #[fail(display = "Invalid ISO8601 period.")]
    ParseError,
    #[fail(display = "Invalid ISO8601 duration.")]
    InvalidDuration,
    #[fail(display = "Invalid ISO8601 datetime.")]
    InvalidDatetime(DatetimeError),
}

#[derive(Clone, PartialEq)]
pub enum Period {
    // P1Y
    Duration(String),
    // 2018-10-11/2019-10-12
    Range(String, String),
    // 2018-10-11/P1Y
    RangeDateDuration(String, String),
    // P1Y/2018-10-11
    RangeDurationDate(String, String),
}

impl Debug for Period {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Period({})", &self)
    }
}

impl Display for Period {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Period::Duration(ref d) => write!(formatter, "{}", d),
            Period::Range(ref s, ref e) => write!(formatter, "{}/{}", s, e),
            Period::RangeDateDuration(ref s, ref d) => write!(formatter, "{}/{}", s, d),
            Period::RangeDurationDate(ref d, ref e) => write!(formatter, "{}/{}", d, e),
        }
    }
}

fn parse_duration(s: &str) -> Result<Period, PeriodError> {
    lazy_static! {
        static ref DURATIONS: RegexSet = RegexSet::new(&[
            r"^P(\d+Y)?(\d+M)?(\d+D)?$",
            r"^PT(\d+H)?(\d+M)?(\d+S)?$",
            r"^P(\d+Y)?(\d+M)?(\d+D)?T(\d+H)?(\d+M)?(\d+S)?$",
        ]).unwrap();
    }

    if DURATIONS.is_match(s) && s != "P" && s != "PT" && !s.ends_with("T") {
        Ok(Period::Duration(s.to_owned()))
    } else {
        Err(PeriodError::InvalidDuration)
    }
}

/// TODO: Only pattern checks are performed. An out of range periods will be
/// accepted as valid.
impl Parse for Period {
    type Err = PeriodError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, '/').collect();

        match v.len() {
            1 => parse_duration(s),
            2 => {
                if v[0].starts_with("P") {
                    let _d = parse_duration(v[0])?;
                    let _e = Datetime::parse(v[1])?;
                    Ok(Period::RangeDurationDate(v[0].to_owned(), v[1].to_owned()))
                } else if v[1].starts_with("P") {
                    let _s = Datetime::parse(v[0])?;
                    let _d = parse_duration(v[1])?;
                    Ok(Period::RangeDateDuration(v[0].to_owned(), v[1].to_owned()))
                } else {
                    let _s = Datetime::parse(v[0])?;
                    let _e = Datetime::parse(v[1])?;
                    Ok(Period::Range(v[0].to_owned(), v[1].to_owned()))
                }
            }
            _ => Err(PeriodError::ParseError),
        }
    }
}

impl From<DatetimeError> for PeriodError {
    fn from(err: DatetimeError) -> Self {
        PeriodError::InvalidDatetime(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration() {
        let expected = r#"Ok(Period(P1Y))"#.to_string();
        let actual = Period::parse("P1Y");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_range() {
        let expected = r#"Ok(Period(2018-01-02/2018-02-03))"#.to_string();
        let actual = Period::parse("2018-01-02/2018-02-03");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_range_date_duration() {
        let expected = r#"Ok(Period(2018-01-01/P1M))"#.to_string();
        let actual = Period::parse("2018-01-01/P1M");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_range_duration_date() {
        let expected = r#"Ok(Period(PT1H/2018-01-02T10:11:12Z))"#.to_string();
        let actual = Period::parse("PT1H/2018-01-02T10:11:12Z");

        assert_eq!(format!("{:?}", actual), expected);
    }
}
