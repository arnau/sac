// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::{Regex, RegexSet};
use std::fmt::{self, Debug, Display};
use super::Parse;

#[derive(Debug, Fail)]
pub enum PointError {
    #[fail(display = "Invalid WKT point.")]
    ParseError,
}

/// Geo Point
///
/// See: https://en.wikipedia.org/wiki/Well-known_text
///
/// Note: [GeoJSON](https://tools.ietf.org/html/rfc7946) is not used because it diverges from the
/// rest of types in style, in particular it describes the type inline with the data. WKT is more
/// in line with formats like ISO8601 or RFC3339. It also aligns with common
/// systems like PostGIS.
///
/// ```text
/// POINT(0 0)
/// POINTZ(0 0 0)
/// ```
#[derive(Clone, PartialEq)]
pub enum Point {
    Point(f64, f64),
    PointZ(f64, f64, f64),
    PointM(f64, f64, f64), // TODO: Review if we want to express M
    PointZM(f64, f64, f64, f64),
}

impl Point {
    pub fn x(&self) -> f64 {
        match *self {
            Point::Point(x, _) => x,
            Point::PointZ(x, _, _) => x,
            Point::PointM(x, _, _) => x,
            Point::PointZM(x, _, _, _) => x,
        }
    }

    pub fn y(&self) -> f64 {
        match *self {
            Point::Point(_, y) => y,
            Point::PointZ(_, y, _) => y,
            Point::PointM(_, y, _) => y,
            Point::PointZM(_, y, _, _) => y,
        }
    }

    pub fn z(&self) -> Option<f64> {
        match *self {
            Point::PointZ(_, _, z) => Some(z),
            Point::PointZM(_, _, z, _) => Some(z),
            _ => None,
        }
    }

    pub fn m(&self) -> Option<f64> {
        match *self {
            Point::PointM(_, _, m) => Some(m),
            Point::PointZM(_, _, _, m) => Some(m),
            _ => None,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Point::Point(ref x, ref y) => write!(formatter, "POINT({} {})", x, y),
            Point::PointZ(ref x, ref y, ref z) => write!(formatter, "POINTZ({} {} {})", x, y, z),
            _ => unimplemented!(),
        }
    }
}

impl Parse for Point {
    type Err = PointError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_SET: Vec<Regex> = vec![
                Regex::new(r"^POINT\((\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\)$").unwrap(),
                Regex::new(r"^POINTZ\((\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\)$").unwrap(),
            ];
            static ref RE: RegexSet = RegexSet::new(RE_SET.iter().map(|re| re.as_str())).unwrap();
        }

        let ms: Vec<_> = RE.matches(s).into_iter().collect();
        if !ms.is_empty() {
            let idx = ms[0];
            let caps = RE_SET[idx].captures(s).unwrap();

            match idx {
                0 => Ok(Point::Point(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                )),
                1 => Ok(Point::PointZ(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                )),

                _ => unreachable!(),
            }
        } else {
            Err(PointError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_point() {
        let point = Point::parse("POINT(0 0)").unwrap();

        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), None);
    }

    #[test]
    fn parse_pointz() {
        let point = Point::parse("POINTZ(0 0 1)").unwrap();

        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), Some(1.0));
    }

}
