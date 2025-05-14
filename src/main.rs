// our code
mod input;

mod command;
use command::Freq;

#[cfg(feature = "egg")]
mod egg;

mod ordered;

mod build_features;
use build_features::*;

// stdlib
use std::process::exit;

// packages
//use clap::builder::styling::*;
use clap::{CommandFactory, FromArgMatches};

fn main() {
    /*
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default() | Effects::BOLD);
    */

    let matches = Freq::command()
        .disable_version_flag(true)
        .long_version(get_long_version())
        //.styles(styles)
        .get_matches();

    let mut freq = Freq::from_arg_matches(&matches).unwrap();
    // HACK clap doesn't have a way to differentiate long vs short flags, so...
    freq.long_version = !std::env::args()
        .filter(|v| v == "--version")
        .collect::<Vec<_>>()
        .is_empty();

    match freq.exec() {
        Ok(exit_code) => exit(exit_code),
        Err(err) => {
            eprintln!("{}", err);
            exit(err.exit_code());
        },
    }
}
