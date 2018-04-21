#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

extern crate sac;

use sac::item;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("sac")
        .version(crate_version!())
        .about("Registers toolbelt")
        .author("Arnau Siches")
        .subcommand(
            SubCommand::with_name("item")
                .about("Manage items")
                .subcommand(
                    SubCommand::with_name("canon")
                        .about("Canonicalise item")
                        .arg(
                            Arg::with_name("input")
                                .help("The item as JSON")
                                .required(true)
                                .index(1),
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
                    Err(err) => println!("{}", err.message),
                }
            }
            _ => unimplemented!(),
        },

        _ => unimplemented!(),
    }
}
