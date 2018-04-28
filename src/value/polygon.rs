// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug};

#[derive(Clone, PartialEq)]
pub struct Polygon(pub Vec<f64>);

impl Debug for Polygon {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Polygon").field(&self.0).finish()
    }
}
