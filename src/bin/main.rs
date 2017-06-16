extern crate dtlib;
extern crate clap;

use clap::*;
use dtlib::*;

fn main() {
    let yaml = load_yaml!("opts.yml");
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("uid") {
        user::generate(matches.value_of("INPUT").unwrap())
    } else {
        panic!("No arguments or subcommands provided")
    }
}
