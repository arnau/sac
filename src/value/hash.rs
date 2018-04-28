// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

#[derive(Clone, PartialEq, Debug)]
pub struct Hash(String);

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
