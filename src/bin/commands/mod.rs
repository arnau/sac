use sac::{digest, item};
use failure::{Error, ResultExt};

pub fn item_canon_command(raw: &str) -> Result<String, Error> {
    let item = item::from_json(raw).context("Error while consuming json")?;
    let s = item::to_json(&item).context("Error serialising json")?;

    Ok(s)
}

pub fn item_hash_command(raw: &str, force_flag: bool) -> Result<String, Error> {
    let item = item::from_json(raw).context("Error while consuming json")?;
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
