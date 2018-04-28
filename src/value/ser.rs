// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use serde::ser::{Serialize, Serializer};

use super::Value;

// TODO: Find a way to uppercase HEX here.
// See https://docs.serde.rs/src/serde_json/ser.rs.html#1395-1415
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
