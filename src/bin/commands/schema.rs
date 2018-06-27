// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use failure::Fail;
use sac::schema::Schema;

pub fn check(path: &str) -> Result<(), String> {
    Schema::open(path).map(|_| ()).map_err(|e| {
        if let Some(cause) = e.cause() {
            cause.to_string()
        } else {
            e.to_string()
        }
    })
}
