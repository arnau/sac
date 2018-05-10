// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::str::FromStr;
use std::fmt::{self, Debug, Display};
use super::Parse;

#[derive(Debug, Fail)]
pub enum HashError {
    #[fail(display = "Invalid hash")]
    Invalid,
    #[fail(display = "Invalid algorithm")]
    InvalidAlgorithm,
    #[fail(display = "Invalid value")]
    InvalidValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Alg {
    Sha2256,
}

impl FromStr for Alg {
    type Err = HashError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha-256" => Ok(Alg::Sha2256),
            _ => Err(HashError::InvalidAlgorithm),
        }
    }
}

impl Display for Alg {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Alg::Sha2256 => Display::fmt("sha-256", formatter),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Hash {
    alg: Alg,
    // TODO: The use of &[u8] makes FromStr unusable because conflicting
    // lifetimes. A solution could be to go back to Parse.
    bytes: String,
}

impl Hash {
    pub fn new(alg: Alg, bytes: String) -> Self {
        Hash {
            alg: alg,
            bytes: bytes,
        }
    }
}

impl Debug for Hash {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_tuple("Hash")
            .field(&self.alg)
            .field(&self.bytes)
            .finish()
    }
}

impl Display for Hash {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&format!("{}:{}", &self.alg, &self.bytes), formatter)
    }
}

impl Parse for Hash {
    type Err = HashError;
    fn parse(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();

        if v.len() != 2 {
            Err(HashError::Invalid)
        } else {
            let alg = v[0].parse::<Alg>()?;
            let bytes = v[1].to_owned();

            if bytes.chars().all(is_hex) {
                Ok(Hash::new(alg, bytes))
            } else {
                Err(HashError::InvalidValue)
            }
        }
    }
}

fn is_hex(c: char) -> bool {
    match c {
        '0'...'9' => true,
        'a'...'f' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let hash = Hash::parse(
            "sha-256:129332749e67eb9ab7390d7da2e88173367d001ac3e9e39f06e41690cd05e3ae",
        ).unwrap();

        assert_eq!(hash.alg, Alg::Sha2256);
        assert_eq!(
            hash.bytes,
            "129332749e67eb9ab7390d7da2e88173367d001ac3e9e39f06e41690cd05e3ae"
        );
    }

    #[test]
    fn fail_with_upper_hex() {
        let hash =
            Hash::parse("sha-256:129332749E67EB9AB7390D7DA2E88173367D001AC3E9E39F06E41690CD05E3AE");

        assert_eq!(format!("{:?}", hash), "Err(InvalidValue)".to_owned());
    }

    #[test]
    fn fail_invalid_alg() {
        let hash = Hash::parse(
            "sha-sha-sha:129332749e67eb9ab7390d7da2e88173367d001ac3e9e39f06e41690cd05e3ae",
        );

        assert_eq!(format!("{:?}", hash), "Err(InvalidAlgorithm)".to_owned());
    }

    #[test]
    fn fail_invalid_pattern() {
        let hash = Hash::parse("129332749e67eb9ab7390d7da2e88173367d001ac3e9e39f06e41690cd05e3ae");

        assert_eq!(format!("{:?}", hash), "Err(Invalid)".to_owned());
    }

}
