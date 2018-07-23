extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate rustsec;

use gumdrop::Options;
use rustsec::{AdvisoryDatabase, Repository};
use std::{env, process::exit};

const MIN_EXPECTED_ADVISORIES: usize = 5;

/// Subcommands
#[derive(Debug, Options)]
enum Opts {
    #[options(help = "show help for a command")]
    Help(HelpOpts),

    #[options(help = "check the advisory DB is well-formed")]
    Check(CheckOpts),
}

/// Options for the `help` command
#[derive(Debug, Default, Options)]
struct HelpOpts {
    #[options(free)]
    commands: Vec<String>,
}

/// Options for the `check` command
#[derive(Debug, Default, Options)]
struct CheckOpts {}

fn main() {
    let args: Vec<_> = env::args().collect();

    let opts = Opts::parse_args_default(&args[1..]).unwrap_or_else(|e| {
        match e.to_string().as_ref() {
            // Show usage if no command name is given or if "help" is given
            "missing command name" => help(&[]),
            string => eprintln!("{}: {}", args[0], string),
        }

        exit(2);
    });

    match opts {
        Opts::Help(opts) => help(&opts.commands),
        Opts::Check(_) => check(),
    }

    exit(0);
}

/// Print help message
fn help(_commands: &[String]) {
    println!("Usage: {} [COMMAND] [OPTIONS]", env::args().next().unwrap());
    println!();
    println!("Available commands:");
    println!();
    println!("{}", Opts::command_list().unwrap());
    println!();
}

fn check() {
    let repo = Repository::open(".").unwrap();

    // Ensure Advisories.toml parses
    let advisory_count = AdvisoryDatabase::from_repository(&repo)
        .unwrap()
        .advisories()
        .count();

    // Ensure we're parsing some advisories
    if advisory_count > MIN_EXPECTED_ADVISORIES {
        println!(
            "*** Check succeeded! Successfully parsed {} advisories.",
            advisory_count
        );
    } else {
        panic!(
            "Missing advisories! Expected at least {}, but got {}",
            MIN_EXPECTED_ADVISORIES, advisory_count
        );
    }
}
