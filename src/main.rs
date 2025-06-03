// our code
mod input;

mod command;
use command::{Freq, FreqArgs};

#[cfg(feature = "egg")]
mod egg;

mod ordered;

mod build_features;
use build_features::*;

// stdlib
use std::process::exit;

// packages
use clap::{Command, CommandFactory};

#[cfg(feature = "color")]
fn apply_styles(command: Command) -> Command {
    use clap::builder::styling::*;

    let styles = Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Yellow.on_default() | Effects::BOLD);

    command.styles(styles)
}

fn main() {
    let command = FreqArgs::command()
        .disable_version_flag(true)
        .long_version(get_long_version());

    #[cfg(feature = "color")]
    let command = apply_styles(command);

    let freq = Freq::from_command(command).unwrap();

    match freq.exec() {
        Ok(exit_code) => exit(exit_code),
        Err(err) => {
            let exit_code = err.exit_code();
            err.print();
            exit(exit_code);
        },
    }
}
