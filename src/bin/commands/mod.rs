use sac::{digest, item};
use failure::Error;

pub fn item_canon(raw: &str) -> Result<String, Error> {
    item::from_json(raw).and_then(|item| item::to_json(&item))
}

pub fn item_hash(raw: &str, force_flag: bool) -> Result<String, Error> {
    let item = item::from_json(raw)?;
    let hash = item.hash();

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
