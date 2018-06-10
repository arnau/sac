// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use super::Parse;
use regex::{Regex, RegexSet};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Fail)]
pub enum PointError {
    #[fail(display = "Invalid WKT point. Valid examples:\n\n  POINT (0 1)\n  POINTZ (1 2 3)\n")]
    ParseError,
    #[fail(display = "Unexpected vector. Expected length 2")]
    UnexpectedVectorLength,
}

pub trait Coord: Clone + Display + Debug {
    type Err;

    fn dimension(&self) -> u8;

    fn from_vec(v: Vec<f64>) -> Result<Self, Self::Err>;
}

#[derive(Clone, PartialEq)]
pub struct Coord2(f64, f64);

impl Coord for Coord2 {
    type Err = PointError;
    fn dimension(&self) -> u8 {
        2
    }

    fn from_vec(v: Vec<f64>) -> Result<Self, Self::Err> {
        if v.len() == 2 {
            Ok(Coord2(v[0], v[1]))
        } else {
            Err(PointError::UnexpectedVectorLength)
        }
    }
}

impl Coord2 {
    pub fn new(x: f64, y: f64) -> Self {
        Coord2(x, y)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
}

impl Debug for Coord2 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl Display for Coord2 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {}", &self.x(), &self.y())
    }
}

#[derive(Clone, PartialEq)]
pub struct Coord3(f64, f64, f64);

impl Coord for Coord3 {
    type Err = PointError;
    fn dimension(&self) -> u8 {
        3
    }

    fn from_vec(v: Vec<f64>) -> Result<Self, Self::Err> {
        if v.len() == 3 {
            Ok(Coord3(v[0], v[1], v[2]))
        } else {
            Err(PointError::UnexpectedVectorLength)
        }
    }
}

impl Coord3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Coord3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl Debug for Coord3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl Display for Coord3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} {} {}", &self.x(), &self.y(), &self.z())
    }
}

/// Geographical Point.
///
/// See: https://en.wikipedia.org/wiki/Well-known_text
///
/// Note: M values are not implemented so only WKT `POINT` and `POINTZ` are accepted when
/// parsing from a string.
///
/// ```text
/// POINT(0 0)
/// POINTZ(0 0 0)
/// ```
#[derive(Clone, PartialEq)]
pub enum Point {
    Point(Coord2),
    PointZ(Coord3),
}

impl Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Point::Point(ref p) => write!(formatter, "POINT ({})", p),
            Point::PointZ(ref p) => write!(formatter, "POINTZ ({})", p),
        }
    }
}

impl Point {
    pub fn dimension(&self) -> u8 {
        match *self {
            Point::Point(ref p) => p.dimension(),
            Point::PointZ(ref p) => p.dimension(),
        }
    }

    pub fn x(&self) -> f64 {
        match *self {
            Point::Point(ref p) => p.x(),
            Point::PointZ(ref p) => p.x(),
        }
    }

    pub fn y(&self) -> f64 {
        match *self {
            Point::Point(ref p) => p.y(),
            Point::PointZ(ref p) => p.y(),
        }
    }

    pub fn z(&self) -> Option<f64> {
        match *self {
            Point::PointZ(ref p) => Some(p.z()),
            _ => None,
        }
    }
}

impl Parse for Point {
    type Err = PointError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_SET: Vec<Regex> = vec![
                Regex::new(r"^POINT\s\((\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\)$").unwrap(),
                Regex::new(r"^POINTZ\s\((\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\s(\d+(?:\.\d+)?)\)$")
                    .unwrap(),
            ];
            static ref RE: RegexSet = RegexSet::new(RE_SET.iter().map(|re| re.as_str())).unwrap();
        }

        let ms: Vec<_> = RE.matches(s).into_iter().collect();
        if !ms.is_empty() {
            let idx = ms[0];
            let caps = RE_SET[idx].captures(s).unwrap();

            match idx {
                0 => Ok(Point::Point(Coord2(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                ))),
                1 => Ok(Point::PointZ(Coord3(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                ))),

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
        let point = Point::parse("POINT (0 0)").unwrap();

        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), None);
    }

    #[test]
    fn parse_pointz() {
        let point = Point::parse("POINTZ (0 0 1)").unwrap();

        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), Some(1.0));
    }

}
