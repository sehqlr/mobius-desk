extern crate git2;
extern crate tempdir;

extern crate clap;
use clap::{Arg, App, SubCommand};

mod repo;

fn main() {
    let matches = App::new("mobius-desk")
        .version("0.1")
        .args_from_usage

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 | _ => println!("Some level of verbosity"),
    }

    if let Some(init_matches) = matches.subcommand_matches("init") {
        let filepath = init_matches.value_of("filepath");
        let new_repo = match repo::create_blank_project(filepath) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
        };
        println!("{:?}", new_repo.state());
    }
}
