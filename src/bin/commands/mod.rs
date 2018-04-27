// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use sac::{blob, digest};
use failure::Error;

pub fn item_canon(raw: &str) -> Result<String, Error> {
    blob::from_json(raw).and_then(|blob| blob::to_json(&blob))
}

pub fn item_hash(raw: &str, force_flag: bool) -> Result<String, Error> {
    let blob = blob::from_json(raw)?;
    let hash = blob.hash();

    if force_flag {
        Ok(hash)
    } else {
        let raw_hash = digest::to_hex(digest::digest(raw).as_ref());

        if raw_hash == hash {
            Ok(hash)
        } else {
            bail!("The given item is not canonical")
        }
    }
}
