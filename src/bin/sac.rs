#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

extern crate sac;

use std::process;
use sac::{digest, item};

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("sac")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Registers toolbelt")
        .subcommand(
            SubCommand::with_name("item")
                .about("Manage items")
                .subcommand(
                    SubCommand::with_name("canon")
                        .aliases(&["fix"])
                        .about("Canonicalise item")
                        .arg(
                            Arg::with_name("input")
                                .help("The item as JSON")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("hash")
                        .about("Hash item")
                        .arg(
                            Arg::with_name("input")
                                .help("The item as JSON")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("force")
                                .help("Forces the item to be canonicalised")
                                .long("force"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("item", Some(item_matches)) => match item_matches.subcommand() {
            ("canon", Some(canon_matches)) => {
                let itemr = value_t!(canon_matches, "input", item::Item);

                match itemr {
                    Ok(item) => println!("{}", item::to_json(&item).unwrap()),
                    Err(err) => {
                        println!("{}", err.message);
                        process::exit(1)
                    }
                }
            }
            ("hash", Some(hash_matches)) => {
                let raw = hash_matches.value_of("input").unwrap();
                let itemr = item::from_json(raw);

                match itemr {
                    Ok(item) => {
                        let hash = item.hash();

                        if hash_matches.is_present("force") {
                            println!("{}", hash);
                        } else {
                            let raw_hash = digest::to_hex(digest::digest(raw).as_ref());

                            if raw_hash == hash {
                                println!("{}", hash);
                            } else {
                                eprintln!("The given item is not canonical");
                                process::exit(1);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
            }

            _ => unimplemented!(),
        },
        ("", None) => {
            println!("No subcommand was used");
            unimplemented!()
        }
        _ => unreachable!(),
    }
}
