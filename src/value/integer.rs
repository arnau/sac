// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug, Display};
// use super::Parse;

/// An decimal integer.
/// TODO: spec doesn't allow floating point numbers.
#[derive(Clone, PartialEq)]
pub struct Integer(pub i64);

impl Debug for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Integer").field(&self.0).finish()
    }
}

impl Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

// impl Parse for Integer {
//     type Err = IntegerError;
//     fn parse(s: &str) -> Result<Self, Self::Err> {

//     }
// }
