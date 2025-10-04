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
use clap::builder::StyledStr;

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

#[cfg(not(feature = "color"))]
fn apply_styles(command: Command) -> Command {
    command
}

fn apply_before_help(command: Command) -> Command {
    let mut styled = StyledStr::new();

    let before = format!(
        "{} {}\n{}\n{}\n",
        command.get_name(),
        command.get_version().unwrap(),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_REPOSITORY"),
    );
    styled.push_str(&before);

    command.before_help(styled)
}

fn apply_after_help(command: Command) -> Command {
    #[allow(unused_imports)]
    use clap::builder::styling::*;

    let mut styled = StyledStr::new();
    let styles = command.get_styles();
    let header = styles.get_header();
//    let part = format!("{header}{}{header:#}", "hello");
//    styled.push_str(&part);

    command.after_help(styled)
}

fn main() {
    let command = FreqArgs::command()
        .disable_version_flag(true)
        .long_version(get_long_version());

    let command = apply_styles(command);
    let command = apply_before_help(command);
    let command = apply_after_help(command);

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
