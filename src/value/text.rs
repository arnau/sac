// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::fmt::{self, Debug};

#[derive(Clone, PartialEq)]
pub struct Text(String);

impl Debug for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_tuple("Text").field(&self.0).finish()
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
