// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::{Regex, RegexSet};
use std::fmt::{self, Debug, Display};
use super::Parse;

#[derive(Debug, Fail)]
pub enum DatetimeError {
    #[fail(display = "Invalid ISO8601 datetime.")]
    ParseError,
}

/// ISO8601 Date time
#[derive(Clone, PartialEq)]
pub enum Datetime {
    Year(u16),
    YearMonth(u16, u8),
    Date(u16, u8, u8),
    DateHour(u16, u8, u8, u8),
    DateHourMinute(u16, u8, u8, u8, u8),
    Full(u16, u8, u8, u8, u8, u8),
}

impl Debug for Datetime {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Datetime({})", &self)
    }
}

impl Display for Datetime {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Datetime::Year(ref y) => write!(formatter, "{:04}", y),
            Datetime::YearMonth(ref y, ref m) => write!(formatter, "{:04}-{:02}", y, m),
            Datetime::Date(ref y, ref m, ref d) => write!(formatter, "{:04}-{:02}-{:02}", y, m, d),
            Datetime::DateHour(ref y, ref m, ref d, ref h) => {
                write!(formatter, "{:04}-{:02}-{:02}T{:02}Z", y, m, d, h)
            }
            Datetime::DateHourMinute(ref y, ref m, ref d, ref h, ref mm) => {
                write!(formatter, "{:04}-{:02}-{:02}T{:02}:{:02}Z", y, m, d, h, mm)
            }

            Datetime::Full(ref y, ref m, ref d, ref h, ref mm, ref s) => write!(
                formatter,
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                y, m, d, h, mm, s
            ),
        }
    }
}

/// TODO: Only pattern checks are performed. An out of range date will be
/// accepted as valid.
impl Parse for Datetime {
    type Err = DatetimeError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_SET: Vec<Regex> = vec![
                Regex::new(r"^(?P<year>\d{4})$").unwrap(),
                Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})$").unwrap(),
                Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$").unwrap(),
                Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2})Z$").unwrap(),
                Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2})Z$").unwrap(),
                Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})Z$").unwrap(),
            ];

            static ref RE: RegexSet = RegexSet::new(RE_SET.iter().map(|re| re.as_str())).unwrap();
        }

        let ms: Vec<_> = RE.matches(s).into_iter().collect();
        if !ms.is_empty() {
            let idx = ms[0];
            let caps = RE_SET[idx].captures(s).unwrap();

            match idx {
                0 => Ok(Datetime::Year((&caps["year"]).parse().unwrap())),

                1 => Ok(Datetime::YearMonth(
                    (&caps["year"]).parse().unwrap(),
                    (&caps["month"]).parse().unwrap(),
                )),

                2 => Ok(Datetime::Date(
                    (&caps["year"]).parse().unwrap(),
                    (&caps["month"]).parse().unwrap(),
                    (&caps["day"]).parse().unwrap(),
                )),
                3 => Ok(Datetime::DateHour(
                    (&caps["year"]).parse().unwrap(),
                    (&caps["month"]).parse().unwrap(),
                    (&caps["day"]).parse().unwrap(),
                    (&caps["hour"]).parse().unwrap(),
                )),
                4 => Ok(Datetime::DateHourMinute(
                    (&caps["year"]).parse().unwrap(),
                    (&caps["month"]).parse().unwrap(),
                    (&caps["day"]).parse().unwrap(),
                    (&caps["hour"]).parse().unwrap(),
                    (&caps["minute"]).parse().unwrap(),
                )),
                5 => Ok(Datetime::Full(
                    (&caps["year"]).parse().unwrap(),
                    (&caps["month"]).parse().unwrap(),
                    (&caps["day"]).parse().unwrap(),
                    (&caps["hour"]).parse().unwrap(),
                    (&caps["minute"]).parse().unwrap(),
                    (&caps["second"]).parse().unwrap(),
                )),

                _ => unreachable!(),
            }
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
        let expected = r#"Ok(Datetime(2018))"#.to_string();
        let actual = Datetime::parse("2018");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_yearmonth() {
        let expected = r#"Ok(Datetime(2018-01))"#.to_string();
        let actual = Datetime::parse("2018-01");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_date() {
        let expected = r#"Ok(Datetime(2018-01-01))"#.to_string();
        let actual = Datetime::parse("2018-01-01");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datehour() {
        let expected = r#"Ok(Datetime(2018-01-01T10Z))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datehourminute() {
        let expected = r#"Ok(Datetime(2018-01-01T10:11Z))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10:11Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

    #[test]
    fn parse_datetime() {
        let expected = r#"Ok(Datetime(2018-01-01T10:11:12Z))"#.to_string();
        let actual = Datetime::parse("2018-01-01T10:11:12Z");

        assert_eq!(format!("{:?}", actual), expected);
    }

}
