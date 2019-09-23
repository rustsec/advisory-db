//! RustSec Advisory DB Linter

#![allow(clippy::never_loop)]

use gumdrop::Options;
use std::{env, io::Write, path::PathBuf, process::exit};
use termcolor::{
    Color::{Green, Red},
    ColorChoice, ColorSpec, StandardStream, WriteColor,
};

/// Minimum number of advisories we expect in the database
const MIN_EXPECTED_ADVISORIES: usize = 10;

macro_rules! writeln_color {
    ($stream:expr, $color:path, $fmt:expr, $msg:expr) => {
        let mut color = ColorSpec::new();
        color.bold();
        color.set_fg(Some($color));
        $stream.set_color(&color).unwrap();

        writeln!($stream, $fmt, $msg).unwrap();
        $stream.reset().unwrap();
    };
}

macro_rules! writeln_success {
    ($stream:expr, $msg:expr) => {
        writeln_color!($stream, Green, "✔ {}", $msg);
    };
    ($stream:expr, $fmt:expr, $($arg:tt)+) => {
        writeln_success!($stream, format!($fmt, $($arg)+));
    }
}

macro_rules! writeln_error {
    ($stream:expr, $msg:expr) => {
        writeln_color!($stream, Red, "✘ {}", $msg);
    };
    ($stream:expr, $fmt:expr, $($arg:tt)+) => {
        writeln_error!($stream, format!($fmt, $($arg)+));
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let opts = Commands::parse_args_default(&args[1..]).unwrap_or_else(|e| {
        match e.to_string().as_ref() {
            // Show usage if no command name is given or if "help" is given
            "missing command name" => help(),
            string => eprintln!("{}: {}", args[0], string),
        }

        exit(1);
    });

    match opts {
        Commands::Help(_) => help(),
        Commands::Check(check) => check.run(),
    }
}

/// Subcommands
#[derive(Debug, Options)]
enum Commands {
    #[options(help = "show help for a command")]
    Help(HelpCommand),

    #[options(help = "check the advisory DB is well-formed")]
    Check(CheckCommand),
}

/// Options for the `help` command
#[derive(Debug, Default, Options)]
struct HelpCommand {
    #[options(free)]
    commands: Vec<String>,
}

/// Print help message
fn help() {
    println!("Usage: {} [COMMAND] [OPTIONS]", env::args().next().unwrap());
    println!();
    println!("Available commands:");
    println!();
    println!("{}", Commands::command_list().unwrap());
    println!();
}

/// Options for the `check` command
#[derive(Debug, Default, Options)]
struct CheckCommand {}

impl CheckCommand {
    fn run(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let repo = rustsec::Repository::open(".").unwrap();

        // Ensure Advisories.toml parses
        let db = rustsec::Database::load(&repo).unwrap();
        let advisories = db.iter();

        // Ensure we're parsing some advisories
        if advisories.len() > MIN_EXPECTED_ADVISORIES {
            writeln_success!(
                &mut stdout,
                "Successfully parsed {} advisories",
                advisories.len()
            );
        } else {
            writeln_error!(
                &mut stdout,
                "Missing advisories! Expected at least {}, but got {}",
                MIN_EXPECTED_ADVISORIES,
                advisories.len()
            );
            exit(1);
        }

        let cratesio_client = crates_io_api::SyncClient::new();

        let mut invalid_advisories = 0;

        for advisory in advisories {
            if !self.check_advisory(&mut stdout, &cratesio_client, advisory) {
                invalid_advisories += 1;
            }
        }

        if invalid_advisories == 0 {
            writeln_success!(&mut stdout, "All advisories are well-formed");
        } else {
            writeln_error!(
                &mut stdout,
                "{} advisories contain errors!",
                invalid_advisories
            );
            exit(1);
        }
    }

    fn check_advisory(
        &self,
        stdout: &mut StandardStream,
        cratesio_client: &crates_io_api::SyncClient,
        advisory: &rustsec::Advisory,
    ) -> bool {
        if advisory.metadata.collection == Some(rustsec::Collection::Crates) {
            match cratesio_client.get_crate(advisory.metadata.package.as_str()) {
                Ok(response) => {
                    if response.crate_data.name != advisory.metadata.package.as_str() {
                        writeln_error!(
                            stdout,
                            "crates.io package name does not match package name in advisory for {}",
                            advisory.metadata.package.as_str()
                        );
                        return false;
                    }
                }
                Err(err) => {
                    writeln_error!(
                        stdout,
                        "Failed to get package `{}` from crates.io: {}",
                        advisory.metadata.package.as_str(),
                        err
                    );
                    return false;
                }
            }
        }

        let mut advisory_path = PathBuf::from(".")
            .join(advisory.metadata.collection.as_ref().unwrap().to_string())
            .join(advisory.metadata.package.as_str())
            .join(advisory.metadata.id.as_str());

        advisory_path.set_extension("toml");

        let lint = rustsec::advisory::Linter::lint_file(&advisory_path).unwrap();

        if lint.errors().is_empty() {
            writeln_success!(
                stdout,
                "{} successfully passed lint",
                advisory_path.display()
            );
            true
        } else {
            writeln_error!(
                stdout,
                "{} contained the following lint errors:",
                advisory_path.display()
            );

            for error in lint.errors() {
                writeln!(stdout, "  - {}", error).unwrap();
            }

            false
        }
    }
}
