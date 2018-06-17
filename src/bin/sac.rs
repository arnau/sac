// Copyright 2018 Arnau Siches
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate log;

extern crate sac;
mod commands;
use std::process;

use clap::{App, Arg, SubCommand};

use sac::kind::Kind;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Registers toolbelt")
        .subcommand(
            SubCommand::with_name("blob")
                .about("Manage blobs")
                .subcommand(
                    SubCommand::with_name("canon")
                        .aliases(&["fix"])
                        .about("Canonicalise blob")
                        .arg(
                            Arg::with_name("input")
                                .help("The blob as JSON")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("hash")
                        .about("Compute the hash of the given blob")
                        .arg(
                            Arg::with_name("input")
                                .help("The blob as JSON")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("force")
                                .help("Forces the blob to be canonicalised")
                                .long("force"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("value")
                .about("Operate on values")
                .subcommand(
                    SubCommand::with_name("check")
                        .about("Check if a value is valid")
                        .arg(
                            Arg::with_name("input")
                                .help("The value to be checked")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("type")
                                .help("The type the value is expected to be")
                                .long("type")
                                .short("t")
                                .takes_value(true)
                                .required(true)
                                .possible_values(&[
                                    "bool",
                                    "curie",
                                    "datetime",
                                    "hash",
                                    "inapplicable",
                                    "integer",
                                    "period",
                                    "point",
                                    "polygon",
                                    "string",
                                    "text",
                                    "timestamp",
                                    "unknown",
                                    "untyped",
                                    "url",
                                ]),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("blob", Some(cmd_matches)) => match cmd_matches.subcommand() {
            ("canon", Some(sub_matches)) => {
                let raw = sub_matches.value_of("input").unwrap();

                match commands::item_canon(raw) {
                    Ok(json) => println!("{}", json),
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(1)
                    }
                }
            }
            ("hash", Some(sub_matches)) => {
                let raw = sub_matches.value_of("input").unwrap();
                let force_flag = sub_matches.is_present("force");

                match commands::item_hash(raw, force_flag) {
                    Ok(hash) => println!("{}", hash),
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(1)
                    }
                }
            }

            _ => unimplemented!(),
        },
        ("value", Some(value_matches)) => match value_matches.subcommand() {
            ("check", Some(sub_matches)) => {
                let raw = sub_matches.value_of("input").unwrap();
                let kind = value_t!(sub_matches, "type", Kind).unwrap();

                match commands::value::check(raw, kind.clone()) {
                    Ok(v) => println!("The value {} is a valid {}", v, kind),
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(1)
                    }
                }
            }
            _ => process::exit(127),
        },
        ("", None) => {
            println!("No subcommand was used");
            unimplemented!()
        }
        // Unknown command
        _ => process::exit(127),
    }
}
