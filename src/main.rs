#![allow(clippy::never_loop)]

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
    let db = AdvisoryDatabase::from_repository(&repo).unwrap();
    let advisories = db.advisories();

    // Ensure we're parsing some advisories
    if advisories.len() > MIN_EXPECTED_ADVISORIES {
        println!(
            "*** Check succeeded! Successfully parsed {} advisories.",
            advisories.len()
        );
    } else {
        panic!(
            "Missing advisories! Expected at least {}, but got {}",
            MIN_EXPECTED_ADVISORIES,
            advisories.len()
        );
    }

    let cratesio_client = crates_io_api::SyncClient::new();
    for advisory in advisories {
        check_advisory(&cratesio_client, advisory);
    }
    println!("*** Check succeeded! All advisories refer to valid crates.");
}

fn check_advisory(cratesio_client: &crates_io_api::SyncClient, advisory: &rustsec::Advisory) {
    let response = cratesio_client
        .get_crate(advisory.package.as_str())
        .unwrap_or_else(|_| {
            panic!(
                "Failed to get package from crates.io: {}",
                advisory.package.as_str()
            )
        });

    if response.crate_data.name != advisory.package.as_str() {
        panic!(
            "crates.io package name does not match package name in advisory for {}",
            advisory.package.as_str()
        );
    }

    // Check that each path in `affected_paths` starts with the crate name
    if let Some(ref version_req_paths) = advisory.affected_paths {
        for (_, paths) in version_req_paths.iter() {
            for path in paths {
                if path.crate_name() != response.crate_data.name {
                    panic!(
                        "{}: affected_path does not begin with crate name: {}",
                        response.crate_data.name,
                        path.crate_name()
                    )
                }
            }
        }
    }
}
