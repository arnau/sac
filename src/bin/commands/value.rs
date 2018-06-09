// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use failure::Fail;
use sac::kind::Kind;
use sac::value::Value;

pub fn check(raw: &str, kind: Kind) -> Result<String, String> {
    Value::parse(raw, kind).map(|x| x.to_string()).map_err(|e| {
        if let Some(cause) = e.cause() {
            cause.to_string()
        } else {
            e.to_string()
        }
    })
}
