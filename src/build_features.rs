include!(concat!(env!("OUT_DIR"),"/build_features.rs"));

build_info::build_info!(fn binfo);

pub fn get_long_version() -> &'static str {
    let info = binfo();
    let mut output = String::from(LONG_VERSION);

    output.push_str(" (");
    output.push_str(&info.target.triple);
    if PROFILE == "debug" {
        output.push_str(", debug");
    }
    output.push_str(")");

    output.push_str(build_info::format!(
        "\nBuilt at {} with {}",
        $.timestamp,
        $.compiler,
    ));

    match info.crate_info.authors.len() {
        0 => (),
        1 => output.push_str(&format!("\nAuthor: {}", info.crate_info.authors[0])),
        _ => output.push_str(&format!("\nAuthors: {}", info.crate_info.authors.join("; "))),
    }

    #[allow(clippy::const_is_empty)]
    if !FEATURES.is_empty() {
        output.push_str(&format!(
            "\nFeatures: {}",
            FEATURES.join(" "),
        ));
    } else {
        output.push_str("\nFeatures: None");
    }

    Box::leak(output.into_boxed_str())
}
