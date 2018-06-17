// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use super::point::{Coord, Coord2, Coord3};
use super::Parse;
use regex::{Regex, RegexSet};
use std::fmt::{self, Debug, Display};
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug, Fail)]
pub enum PolygonError {
    #[fail(
        display = "Invalid WKT polygon. Valid examples:\n\n  POLYGON ((0 10, 10 20, 20 0))\n  POLYGON ((0 10, 10 20, 20 0), (2 8, 8 18, 18 2))\n  POLYGONZ ((0 0 1, 1 1 1, 2 2 1))\n"
    )]
    ParseError,
    #[fail(display = "Unexpected number in polygon.")]
    ParseFloatError(#[cause] ParseFloatError),
    #[fail(display = "Expected {} polygon coordinates.", _0)]
    InvalidCoord(String),
}

impl From<ParseFloatError> for PolygonError {
    fn from(err: ParseFloatError) -> PolygonError {
        PolygonError::ParseFloatError(err)
    }
}

#[derive(Clone, PartialEq)]
pub struct Ring<T: Coord>(Vec<T>);

impl<T: Coord> Ring<T> {
    pub fn new(v: Vec<T>) -> Self {
        Ring(v)
    }

    pub fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: Coord> FromStr for Ring<T> {
    type Err = PolygonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = parse_numbers(s)?;

        let vec = elements
            .into_iter()
            .map(|x| match x.len() {
                2 => T::from_vec(x).map_err(|_| PolygonError::InvalidCoord("3D".into())),
                3 => T::from_vec(x).map_err(|_| PolygonError::InvalidCoord("2D".into())),
                _ => Err(PolygonError::InvalidCoord("2D or 3D".into())),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Ring::new(vec))
    }
}

fn parse_numbers(s: &str) -> Result<Vec<Vec<f64>>, ParseFloatError> {
    s.split(',')
        .map(|token| token.trim().split(' ').map(|n| n.parse::<f64>()).collect())
        .collect()
}

impl<T: Coord> Debug for Ring<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl<T: Coord> Display for Ring<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(")?;

        for (count, value) in self.0.iter().enumerate() {
            if count != 0 {
                write!(formatter, ", ")?;
            }
            write!(formatter, "{}", value)?;
        }

        write!(formatter, ")")
    }
}

/// Geo Polygon
///
/// See: https://en.wikipedia.org/wiki/Well-known_text
///
/// ## Examples
///
/// ```text
/// POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))
/// POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))
/// POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30), (10 10, 20 20, 30 30))
/// ```
///
/// Inner rings should not cross each other nor the outer ring. This implementation
/// does not check this rule.
#[derive(Clone, PartialEq)]
pub enum Polygon {
    Polygon {
        outer_ring: Ring<Coord2>,
        inner_rings: Vec<Ring<Coord2>>,
    },
    PolygonZ {
        outer_ring: Ring<Coord3>,
        inner_rings: Vec<Ring<Coord3>>,
    },
}

impl Debug for Polygon {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self)
    }
}

impl Display for Polygon {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Polygon::Polygon {
                ref outer_ring,
                ref inner_rings,
            } => {
                write!(formatter, "POLYGON ({}", outer_ring)?;

                for inner_ring in inner_rings {
                    write!(formatter, ", ")?;
                    write!(formatter, "{}", inner_ring)?;
                }

                write!(formatter, ")")
            }
            Polygon::PolygonZ {
                ref outer_ring,
                ref inner_rings,
            } => {
                write!(formatter, "POLYGONZ ({}", outer_ring)?;

                for inner_ring in inner_rings {
                    write!(formatter, ", ")?;
                    write!(formatter, "{}", inner_ring)?;
                }

                write!(formatter, ")")
            }
        }
    }
}

impl Polygon {
    pub fn new(outer_ring: Ring<Coord2>, inner_rings: Vec<Ring<Coord2>>) -> Self {
        Polygon::Polygon {
            outer_ring,
            inner_rings,
        }
    }

    pub fn newz(outer_ring: Ring<Coord3>, inner_rings: Vec<Ring<Coord3>>) -> Self {
        Polygon::PolygonZ {
            outer_ring,
            inner_rings,
        }
    }
}

fn parse_rings<T: Coord>(s: &str) -> Result<(Ring<T>, Vec<Ring<T>>), PolygonError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\((.+?)\)(?:,\s)?)").unwrap();
    }

    let rings = RE.captures_iter(s)
        .map(|caps| {
            let raw = &caps[1].parse::<String>().unwrap();
            raw.parse::<Ring<T>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    if let Some((outer_ring, inner_rings)) = rings.split_first() {
        Ok((outer_ring.clone(), inner_rings.to_vec()))
    } else {
        Err(PolygonError::ParseError)
    }
}

impl Parse for Polygon {
    type Err = PolygonError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_SET: Vec<Regex> = vec![
                Regex::new(r"^POLYGON\s\((.+)\)$").unwrap(),
                Regex::new(r"^POLYGONZ\s\((.+)\)$").unwrap(),
            ];
            static ref RE: RegexSet = RegexSet::new(RE_SET.iter().map(|re| re.as_str())).unwrap();
        }

        let ms: Vec<_> = RE.matches(s).into_iter().collect();
        if !ms.is_empty() {
            let idx = ms[0];
            let caps = RE_SET[idx].captures(s).unwrap();

            match idx {
                0 => {
                    let (outer_ring, inner_rings) =
                        parse_rings::<Coord2>(&caps[1].parse::<String>().unwrap())?;

                    Ok(Polygon::new(outer_ring, inner_rings))
                }
                1 => {
                    let (outer_ring, inner_rings) =
                        parse_rings::<Coord3>(&caps[1].parse::<String>().unwrap())?;

                    Ok(Polygon::newz(outer_ring, inner_rings))
                }
                _ => unreachable!(),
            }
        } else {
            Err(PolygonError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let actual = Polygon::parse("POLYGON ()");

        assert!(
            actual.is_err(),
            "Expect an empty polygon to return a parse error"
        );
    }

    #[test]
    fn parse_polygon_outer() {
        let actual = Polygon::parse("POLYGON ((0 0, 10 10, 20 20))").ok();

        let expected = Polygon::new(
            Ring::new(vec![
                Coord2::new(0.0, 0.0),
                Coord2::new(10.0, 10.0),
                Coord2::new(20.0, 20.0),
            ]),
            vec![],
        );

        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn parse_polygonz_outer() {
        let actual = Polygon::parse("POLYGONZ ((0 0 1, 10 10 1, 20 20 1))").ok();

        let expected = Polygon::newz(
            Ring::new(vec![
                Coord3::new(0.0, 0.0, 1.0),
                Coord3::new(10.0, 10.0, 1.0),
                Coord3::new(20.0, 20.0, 1.0),
            ]),
            vec![],
        );

        assert_eq!(actual, Some(expected));
    }

    mod rings2 {
        use super::*;

        #[test]
        fn test_empty() {
            let actual = parse_rings::<Coord2>("");

            assert!(
                actual.is_err(),
                "Expect an empty string to return a parse error"
            );
        }

        #[test]
        fn test_multiple_inner() {
            let actual = parse_rings::<Coord2>("(0 0, 1 1, 2 2), (3 3, 4 4, 5 5), (1 2, 3 4, 5 6)");
            let expected = (
                Ring::new(vec![
                    Coord2::new(0.0, 0.0),
                    Coord2::new(1.0, 1.0),
                    Coord2::new(2.0, 2.0),
                ]),
                vec![
                    Ring::new(vec![
                        Coord2::new(3.0, 3.0),
                        Coord2::new(4.0, 4.0),
                        Coord2::new(5.0, 5.0),
                    ]),
                    Ring::new(vec![
                        Coord2::new(1.0, 2.0),
                        Coord2::new(3.0, 4.0),
                        Coord2::new(5.0, 6.0),
                    ]),
                ],
            );

            assert_eq!(actual.ok(), Some(expected));
        }

        #[test]
        fn test_outer() {
            let actual = parse_rings::<Coord2>("(0 0, 1 1, 2 2)");
            let expected = (
                Ring::new(vec![
                    Coord2::new(0.0, 0.0),
                    Coord2::new(1.0, 1.0),
                    Coord2::new(2.0, 2.0),
                ]),
                vec![],
            );

            assert_eq!(actual.ok(), Some(expected));
        }

        #[test]
        fn test_single_inner() {
            let actual = parse_rings::<Coord2>("(0 0, 1 1, 2 2), (3 3, 4 4, 5 5)");
            let expected = (
                Ring::new(vec![
                    Coord2::new(0.0, 0.0),
                    Coord2::new(1.0, 1.0),
                    Coord2::new(2.0, 2.0),
                ]),
                vec![Ring::new(vec![
                    Coord2::new(3.0, 3.0),
                    Coord2::new(4.0, 4.0),
                    Coord2::new(5.0, 5.0),
                ])],
            );

            assert_eq!(actual.ok(), Some(expected));
        }
    }

    mod rings3 {
        use super::*;

        #[test]
        fn test_empty() {
            let actual = parse_rings::<Coord3>("");

            assert!(
                actual.is_err(),
                "Expect an empty string to return a parse error"
            );
        }

        #[test]
        fn test_multiple_inner() {
            let actual = parse_rings::<Coord3>(
                "(0 0 0, 1 1 0, 2 2 0), (3 3 0, 4 4 0, 5 5 0), (1 2 0, 3 4 0, 5 6 0)",
            );
            let expected = (
                Ring::new(vec![
                    Coord3::new(0.0, 0.0, 0.0),
                    Coord3::new(1.0, 1.0, 0.0),
                    Coord3::new(2.0, 2.0, 0.0),
                ]),
                vec![
                    Ring::new(vec![
                        Coord3::new(3.0, 3.0, 0.0),
                        Coord3::new(4.0, 4.0, 0.0),
                        Coord3::new(5.0, 5.0, 0.0),
                    ]),
                    Ring::new(vec![
                        Coord3::new(1.0, 2.0, 0.0),
                        Coord3::new(3.0, 4.0, 0.0),
                        Coord3::new(5.0, 6.0, 0.0),
                    ]),
                ],
            );

            assert_eq!(actual.ok(), Some(expected));
        }

        #[test]
        fn test_outer() {
            let actual = parse_rings::<Coord3>("(0 0 0, 1 1 0, 2 2 0)");
            let expected = (
                Ring::new(vec![
                    Coord3::new(0.0, 0.0, 0.0),
                    Coord3::new(1.0, 1.0, 0.0),
                    Coord3::new(2.0, 2.0, 0.0),
                ]),
                vec![],
            );

            assert_eq!(actual.ok(), Some(expected));
        }

        #[test]
        fn test_single_inner() {
            let actual = parse_rings::<Coord3>("(0 0 0, 1 1 0, 2 2 0), (3 3 0, 4 4 0, 5 5 0)");
            let expected = (
                Ring::new(vec![
                    Coord3::new(0.0, 0.0, 0.0),
                    Coord3::new(1.0, 1.0, 0.0),
                    Coord3::new(2.0, 2.0, 0.0),
                ]),
                vec![Ring::new(vec![
                    Coord3::new(3.0, 3.0, 0.0),
                    Coord3::new(4.0, 4.0, 0.0),
                    Coord3::new(5.0, 5.0, 0.0),
                ])],
            );

            assert_eq!(actual.ok(), Some(expected));
        }
    }

}
