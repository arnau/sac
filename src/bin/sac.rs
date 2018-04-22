#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate sac;
mod commands;
use std::process;

use clap::{App, Arg, SubCommand};

fn main() {
    // TODO: Move to a man page
    let item_examples = r#"
EXAMPLES

    $ sac item canon '{"foo": "abc", "bar": "xyz"}'
    {"bar":"xyz","foo":"abc"}

    $ sac item hash '{"bar":"xyz","foo":"abc"}'
    5dd4fe3b0de91882dae86b223ca531b5c8f2335d9ee3fd0ab18dfdc2871d0c61
"#;
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Registers toolbelt")
        .subcommand(
            SubCommand::with_name("item")
                .about("Manage items")
                .after_help(item_examples)
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
                        .about("Compute the hash of the given item")
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
        ("", None) => {
            println!("No subcommand was used");
            unimplemented!()
        }
        // Unknown command
        _ => process::exit(127),
    }
}
