// our code
mod input;

mod command;
use command::Freq;

#[cfg(feature = "egg")]
mod egg;

mod ordered;

// stdlib
use std::process::exit;

// packages
//use clap::builder::styling::*;
use clap::{CommandFactory, FromArgMatches};

include!(concat!(env!("OUT_DIR"),"/build_features.rs"));

build_info::build_info!(fn binfo);

fn get_long_version() -> &'static str {
    let info = binfo();
    let version = format!("v{}", info.crate_info.version);
    let mut parts = vec![version.clone()];

    if let Some(vc) = &info.version_control {
        if let Some(git) = &vc.git() {
            if git.dirty || !git.tags.contains(&version) {
                parts.push("+".to_string());
                if let Some(branch) = &git.branch {
                    parts.push(format!("{}.", branch));
                }
                parts.push(git.commit_short_id.to_string());
                if git.dirty {
                    parts.push("-dirty".to_string());
                }
            }
            parts.push(" (".to_string());
            parts.push(info.target.triple.to_string());
            parts.push(", ".to_string());
            parts.push(PROFILE.to_string());
            parts.push(")".to_string());
        }
    }

    parts.push(build_info::format!(
        "\nBuilt at {} with {}",
        $.timestamp,
        $.compiler,
    ).to_string());

    match info.crate_info.authors.len() {
        0 => (),
        1 => parts.push(format!("\nAuthor: {}", info.crate_info.authors[0])),
        _ => parts.push(format!("\nAuthors: {}", info.crate_info.authors.join("; "))),
    }

    #[allow(clippy::const_is_empty)]
    if !FEATURES.is_empty() {
        parts.push(format!(
            "\nFeatures: {}",
            FEATURES.join(" "),
        ));
    }

    Box::leak(parts.join("").into_boxed_str())
}

fn main() {
    /*
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default() | Effects::BOLD);
    */

    let freq = Freq::from_arg_matches(
        &Freq::command()
        .long_version(get_long_version())
        //.styles(styles)
        .get_matches()
    ).unwrap();

    match freq.exec() {
        Ok(exit_code) => exit(exit_code),
        Err(err) => {
            eprintln!("{}", err);
            exit(err.exit_code());
        },
    }
}
