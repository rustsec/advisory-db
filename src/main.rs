extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate reqwest;
extern crate rustsec;
extern crate serde;
#[macro_use]
extern crate serde_derive;

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

    let http_client = reqwest::Client::new();
    for advisory in advisories {
        check_advisory(&http_client, advisory);
    }
    println!("*** Check succeeded! All advisories refer to valid crates.");
}

#[derive(Deserialize)]
struct CratesResponse {
    #[serde(rename = "crate")]
    krate: Crate,
}

#[derive(Deserialize)]
struct Crate {
    name: String,
}

fn check_advisory(http_client: &reqwest::Client, advisory: &rustsec::Advisory) {
    let mut response = http_client
        .get(&format!(
            "https://crates.io/api/v1/crates/{}",
            advisory.package.as_str()
        ))
        .send()
        .expect("Failed to make HTTP request to crates.io");
    if !response.status().is_success() {
        panic!(
            "crates.io returned an HTTP error ({}) trying to lookup the crate '{}', does it exist?",
            response.status(),
            advisory.package.as_str()
        );
    }

    let parsed_response = response
        .json::<CratesResponse>()
        .expect("Failed to parse crates.io response as JSON");
    if parsed_response.krate.name != advisory.package.as_str() {
        panic!(
            "crates.io package name does not match package name in advisory for {}",
            advisory.package.as_str()
        );
    }
}
