use std::{
    env,
    fs::{self, File},
    io::{self, BufWriter, Write, Read},
    path::Path,
};

use git2::{self, Repository, Commit};

fn maybe_write(path: impl AsRef<Path>, content: &[u8]) -> io::Result<bool> {
    // first, try to `stat` the file
    let should_write = if let Ok(metadata) = fs::metadata(&path) {
        // we could stat it, now check if it's the same size as our content
        if content.len() as usize == metadata.len() as usize {
            // size matches, so check whether the content is the same
            if let Ok(mut file) = File::open(&path) {
                let mut data = Vec::<u8>::new();
                if file.read_to_end(&mut data).is_ok() {
                    data != content
                } else {
                    // failed to read the file
                    true
                }
            } else {
                // failed to open the file
                true
            }
        } else {
            // size of the file differs
            true
        }
    } else {
        // couldn't stat the file, maybe it doesn't exist?
        true
    };

    if should_write {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&content)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn has_tag(repo: &Repository, tag_name: &str) -> bool {
    if let Ok(tag_names) = repo.tag_names(Some(tag_name)) {
        !tag_names.is_empty()
    } else {
        false
    }
}

fn is_dirty(repo: &Repository) -> bool {
    let mut opts = git2::StatusOptions::new();
    opts.include_ignored(false);
    //opts.include_untracked(true);
    //opts.include_unmodified(false);

    let statuses = repo.statuses(Some(&mut opts)).unwrap();

    statuses.iter().next().is_some()
}

fn is_tag(repo: &Repository, commit: &Commit, tag_name: &str) -> bool {
    let commit_id = commit.id();

    let mut not_found = true;
    // scan stops when closure returns false
    repo.tag_foreach(|oid, name| {
        if let Some(name) = String::from_utf8_lossy(name).strip_prefix("refs/tags/") {
            not_found = oid != commit_id || name != tag_name;
        }

        not_found
    }).unwrap();

    !not_found
}

fn long_version() -> String {
    let short_version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let mut long_version = String::from(&short_version);

    if let Ok(repo) = Repository::open(env!("CARGO_MANIFEST_DIR")) {
        let head = repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        let commit_short_id = commit.as_object()
            .short_id().unwrap()
            .as_str().unwrap()
            .to_string();

        let branch = if head.is_branch() {
            head.shorthand().map(|s| s.to_string())
        } else {
            None
        };

        let dirty = is_dirty(&repo);
        if dirty || !is_tag(&repo, &commit, &short_version) {
            if has_tag(&repo, &short_version) {
                long_version.push('+');
            } else {
                long_version.push_str("-pre+");
            }

            if let Some(ref branch) = branch {
                long_version.push_str(branch);
                long_version.push('.');
            }

            long_version.push_str(&commit_short_id);

            if dirty {
                long_version.push_str("-dirty");
            }
        }
    }

    long_version
}

fn features() -> Vec<String> {
    // generate feature list
    let hide_features = ["DEFAULT", "DECOMPRESS", "REGEX", "ALL", "FULL"];
    let mut features = env::vars_os().filter_map(|(key, _)| {
        if let Ok(name) = std::str::from_utf8(&key.into_encoded_bytes()) {
            if let Some(name) = name.strip_prefix("CARGO_FEATURE_") {
                if !hide_features.contains(&name) && !name.starts_with("_") {
                    return Some(name.to_string());
                }
            }
        }

        None
    }).collect::<Vec<_>>();

    // remove conflicting features
    if features.contains(&String::from("REGEX_FANCY")) {
        features.retain(|s| s != "REGEX_BASIC");
    }

    // add meta features
    if !features.contains(&String::from("REGEX")) {
        #[allow(clippy::if_same_then_else)]
        if features.contains(&String::from("REGEX_BASIC")) {
            features.push(String::from("REGEX"));
        } else if features.contains(&String::from("REGEX_FANCY")) {
            features.push(String::from("REGEX"));
        }
    }

    // ensure the features are listed in a consisten order
    features.sort();

    features
}

use std::num::NonZeroUsize;
use clap::{CommandFactory, Parser};
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/args.rs"));

fn man_page() -> io::Result<()> {
    let cmd = FreqArgs::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(&out_dir).join(&man.get_filename());
    maybe_write(&path, &buffer)?;

    Ok(())
}

fn readme() -> io::Result<()> {
    // slurp the existing readme
    let readme_path = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
    let readme_str = {
        let mut readme = File::open(&readme_path)?;
        let mut buf = Vec::<u8>::new();
        readme.read_to_end(&mut buf)?;
        String::from_utf8(buf).map_err(io::Error::other)?
    };

    // try to splice the help output into the readme
    let mut state = 0;
    let mut readme_new = Vec::<String>::new();
    for line in readme_str.lines() {
        if state == 0 && line.starts_with("```") {
            state = 1;
        } else if line.starts_with("```") {
            state = 0;
        } else if state == 1 && line.starts_with("Usage: ") {
            state = 2;
            let mut skipping = true;
            let mut cmd = FreqArgs::command();
            cmd.render_help()
                .to_string()
                .lines()
                .filter(|line| { skipping &= !line.starts_with("Usage: "); !skipping })
                .for_each(|line| readme_new.push(line.trim_end().to_string()));
        }

        if state != 2 {
            readme_new.push(line.to_string());
        }
    }

    // ensure there's a trailing newline at the end of the file
    readme_new.push(String::from(""));

    maybe_write(&readme_path, readme_new.join("\n").as_bytes())?;

    Ok(())
}

fn write_env(writer: &mut impl Write, var: &str) -> io::Result<()> {
    writer.write_all(format!(
        "pub const {}: &'static str = {:?};\n",
        var, env::var(var).unwrap(),
    ).as_bytes())
}

fn main() -> io::Result<()> {
    // Calling `build_info_build::build_script` collects all data and makes it
    // available to `build_info::build_info!`  and `build_info::format!` in the
    // main program.
    {
        build_info_build::build_script();
    }

    man_page()?;
    readme()?;

    // generate a file with the code defining feature data
    let mut content = Vec::<u8>::new();

    // "release" or "debug"
    write_env(&mut content, "PROFILE")?;
    // target triple, e.g. "x86_64-unknown-linux-gnu"
    write_env(&mut content, "TARGET")?;

    content.write_all(format!(
        "pub const LONG_VERSION: &'static str = {:?};\n",
        long_version(),
    ).as_bytes())?;

    content.write_all(format!(
        "pub const FEATURES: &[&str] = &{:?};\n",
        features(),
    ).as_bytes())?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("build_features.rs");
    maybe_write(&path, &content)?;

    // output file implicitly closed
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
