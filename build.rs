use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

use git2::{self, Repository, Commit};

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
    let res = repo.tag_foreach(|oid, name| {
        if let Some(name) = String::from_utf8_lossy(name).strip_prefix("refs/tags/") {
            not_found = oid != commit_id || name != tag_name;
        }

        not_found
    });

    !not_found
}

//fn long_version(repo: &Repository) -> String {
//
//}

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

fn main() {
    // Calling `build_info_build::build_script` collects all data and makes it
    // available to `build_info::build_info!`  and `build_info::format!` in the
    // main program.
    {
        build_info_build::build_script();
    }

    // prepare to write a file with the code defining feature data
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_features.rs");
    let file = File::create(&dest_path).unwrap();
    let mut writer = BufWriter::new(file);

    // "release" or "debug"
    writer.write_all(format!(
        "pub const PROFILE: &str = {:?};\n",
        env::var("PROFILE").unwrap(),
    ).as_bytes()).unwrap();

    writer.write_all(format!(
        "pub const FEATURES: &[&str] = &{:?};\n",
        features(),
    ).as_bytes()).unwrap();

    let repo = Repository::open(env!("CARGO_MANIFEST_DIR")).unwrap();

    let version_tag = format!("v{}", env!("CARGO_PKG_VERSION"));
    let dirty = is_dirty(&repo);
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();
    let mut long_version = String::from(&version_tag);
    if dirty || !is_tag(&repo, &commit, &version_tag) {
        if has_tag(&repo, &version_tag) {
            long_version.push('+');
        } else {
            long_version.push('-');
        }
    }

    writer.write_all(format!(
        "pub const TAGGED: bool = {};\n",
        has_tag(&repo, &version_tag),
    ).as_bytes()).unwrap();

    writer.write_all(format!(
        "pub const LONG_VERSION: &'static str = {:?} {:?};\n",
        long_version,
        false,
    ).as_bytes()).unwrap();

    // output file implicitly closed
    println!("cargo::rerun-if-changed=build.rs");
}
