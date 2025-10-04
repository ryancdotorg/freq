use std::{
    env, fmt,
    fs::{self, File},
    io::{self, BufWriter, Write, Read},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use chrono::{DateTime, Utc};
use git2::{self, Repository, Commit};

fn maybe_write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> io::Result<bool> {
    let content = content.as_ref();

    // Check if we need to write the file by comparing with existing content
    let should_write = match fs::metadata(&path) {
        // File doesn't exist or can't be accessed - write it
        Err(_) => true,

        // File exists, check if size matches
        Ok(metadata) if content.len() != metadata.len() as usize => true,

        // Size matches, compare content, assume we should write if read fails
        Ok(_) => fs::read(&path).map_or(true, |existing| existing != content),
    };

    if should_write {
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

//
// GIT HELPERS
//

fn has_tag(repo: &Repository, tag_name: &str) -> bool {
    if let Ok(tag_names) = repo.tag_names(Some(tag_name)) {
        !tag_names.is_empty()
    } else {
        false
    }
}

fn err_git2<T: fmt::Display>(e: T) -> git2::Error {
    git2::Error::from_str(&e.to_string())
}

struct RepoWalker<'a> {
    repo: &'a Repository,
    stack: Vec<fs::ReadDir>,
}

impl<'a> RepoWalker<'a> {
    fn new(repo: &'a Repository) -> Result<Self, git2::Error> {
        let root = repo.path().parent().unwrap_or(repo.path()).to_path_buf();
        let dir = fs::read_dir(&root).map_err(err_git2)?;
        let stack = vec![dir];
        Ok(RepoWalker { repo, stack })
    }
}

impl<'a> Iterator for RepoWalker<'a> {
    type Item = Result<PathBuf, git2::Error>;

    // Recursively iterate over all the files in the repo, skipping over any
    // which are excluded by .gitignore rules.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entries) = self.stack.last_mut() {
            let Some(entry) = entries.next() else {
                self.stack.pop();
                continue;
            };

            let path = match entry {
                Err(e) => return Some(Err(err_git2(e))),
                Ok(v) => v.path(),
            };

            let is_ignored = match self.repo.is_path_ignored(&path) {
                Err(e) => return Some(Err(e)),
                Ok(v) => v,
            };

            if is_ignored {
                continue;
            }

            if !path.is_dir() {
                return Some(Ok(path.to_owned()));
            }

            match fs::read_dir(&path) {
                Err(e) => return Some(Err(err_git2(e))),
                Ok(dir) => self.stack.push(dir),
            }
        }

        None
    }
}

fn latest_change(repo: &Repository) -> Result<Option<DateTime<Utc>>, git2::Error> {
    Ok(RepoWalker::new(repo)?
        .filter_map(|entry| {
            let path = entry.ok()?;
            let metadata = path.metadata().ok()?;
            let mtime = metadata.modified().ok()?;
            Some(mtime)
        })
        .fold(None, |acc, mtime| match acc {
            None => Some(mtime),
            Some(acc) if mtime > acc => Some(mtime),
            Some(acc) => Some(acc),
        })
        .map(|mtime| mtime.into()))
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

fn get_branch() -> Option<String> {
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR")).ok()?;
    let head = repo.head().ok()?;

    if head.is_branch() { head.shorthand() } else { None }
        .map(clean_semver_ident)
}

//
// VERSION HELPERS
//

static DIRTY_COUNT: OnceLock<u64> = OnceLock::new();

/// Get and update the dirty build count for the current commit
fn dirty_count(commit: &str) -> io::Result<u64> {
    if let Some(count) = DIRTY_COUNT.get() {
        return Ok(*count);
    }

    // build path to tracking file
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/.dirty-build");

    // read and parse previous count data, empty vec on error
    let parts = fs::read(file)
        .map(|s| String::from_utf8_lossy(&s).to_string())
        .map(|s| {
            s.trim_ascii()
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    // calculate new count - if file was valid and commit matches, increment
    // count, otherwise start from 0
    let count = if parts.len() == 2 {
        if parts[0] == commit {
            parts[1].parse().map(|v: u64| v + 1).unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };

    let count = DIRTY_COUNT.get_or_init(|| count);

    // update the tracking file
    fs::write(file, format!("{} {}\n", commit, count))?;

    Ok(*count)
}

fn clean_semver_ident(buildmeta: &str) -> String {
    // From the Semantic Versioning 2.0.0 spec:
    //
    // > Build metadata MAY be denoted by appending a plus sign and a series of dot separated
    // > identifiers immediately following the patch or pre-release version. Identifiers MUST
    // > comprise only ASCII alphanumerics and hyphens [0-9A-Za-z-]. Identifiers MUST NOT be
    // > empty.
    //
    // For now, we just replace disallowed characters with '-'.
    buildmeta
        .chars()
        .map(|c| match c {
            '0'..='9' | 'A'..='Z' | 'a'..='z' | '-' => c,
            _ => '-',
        })
        .collect()
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
                if let Ok(count) = dirty_count(&commit.id().to_string()) {
                    long_version.push_str(&count.to_string());
                }
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

use clap::{Command, CommandFactory};

mod cli {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/cli.rs"));
}
use cli::FreqArgs;

fn get_command() -> Command {
    FreqArgs::command()
        .disable_version_flag(true)
}

fn man_page() -> io::Result<()> {
    let cmd = get_command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(&out_dir).join(man.get_filename());
    maybe_write(&path, &buffer)?;

    Ok(())
}

fn markdown_help() -> io::Result<()> {
    let cmd = get_command();
    let markdown = clap_markdown::help_markdown_command(&cmd);
    let out_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(&out_dir).join("CommandLineHelp.md");
    maybe_write(&path, markdown)?;

    Ok(())
}

fn readme() -> io::Result<()> {
    // slurp the existing readme
    let readme_path = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
    let readme_str = {
        let mut readme = File::open(readme_path)?;
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
            let mut cmd = get_command();
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

    maybe_write(readme_path, readme_new.join("\n"))?;

    Ok(())
}

fn write_env(writer: &mut impl Write, var: &str) -> io::Result<()> {
    writer.write_all(format!(
        "#[allow(dead_code)]\npub const {}: &str = {:?};\n",
        var, env::var(var).unwrap(),
    ).as_bytes())
}

fn write_const_opt_str<K>(writer: &mut impl Write, name: K, value: Option<&str>) -> io::Result<()>
where
    K: fmt::Display,
{
    writer.write_all(format!(
        "#[allow(dead_code)]\npub const {}: Option<&str> = {:?};\n",
        name, value,
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
    markdown_help()?;
    readme()?;

    // generate a file with the code defining feature data
    let mut content = Vec::<u8>::new();

    // "release" or "debug"
    write_env(&mut content, "PROFILE")?;
    // target triple, e.g. "x86_64-unknown-linux-gnu"
    write_env(&mut content, "TARGET")?;

    let mtime = if let Ok(repo) = Repository::open(env!("CARGO_MANIFEST_DIR")) {
        latest_change(&repo).ok().flatten()
    } else {
        None
    };

    if let Some(mtime) = mtime {
        let mtime_str = mtime.format("%Y%m%dT%H%M%SZ").to_string();
        write_const_opt_str(&mut content, "MODIFIED", Some(&mtime_str))?;
    } else {
        write_const_opt_str(&mut content, "MODIFIED", None)?;
    }

    content.write_all(format!(
        "pub const LONG_VERSION: &str = {:?};\n",
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

    if let Some(mtime) = mtime {
        if let Ok(file) = std::fs::File::open(path) {
            // prevent `build_features.rs` from triggering a rebuild
            file.set_modified(mtime.into())?;
        }
    }

    // git stuff
    if let Some(branch) = get_branch() {
        println!("cargo::rerun-if-changed=.git/refs/heads/{branch}");
    }

    Ok(())
}
