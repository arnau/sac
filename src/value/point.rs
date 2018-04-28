// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug};

#[derive(Clone, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_tuple("Point")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
