// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

//! Sac library

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate log;

extern crate pulldown_cmark;

extern crate regex;

extern crate ring;
// https://github.com/RustCrypto/hashes

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate toml;

extern crate url;

pub mod blob;
pub mod digest;
pub mod field;
pub mod kind;
pub mod schema;
pub mod value;
