use std::{env, fs::File, io::{BufWriter, Write}, path::Path};

fn main() {
    // Calling `build_info_build::build_script` collects all data and makes it available to `build_info::build_info!`
    // and `build_info::format!` in the main program.
    {
        build_info_build::build_script();
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_features.rs");
    let file = File::create(&dest_path).unwrap();
    let mut writer = BufWriter::new(file);

    let features: Vec<_> = env::vars_os().filter_map(|(key, _)| {
        if let Ok(name) = std::str::from_utf8(&key.into_encoded_bytes()) {
            if name.starts_with("CARGO_FEATURE_") {
                if name != "CARGO_FEATURE_DEFAULT" {
                    return Some(name[14..].to_string());
                }
            }
        }

        None
    }).collect();

    writer.write_all(format!(
        "const FEATURES: &[&str] = &{:?};\n",
        features,
    ).as_bytes()).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}
