#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author, about, long_about = None)]
pub struct Freq {
    #[arg(short, long, value_name = "FILE", help = "Write output to FILE [default: STDOUT]")]
    output: Option<String>,

    #[cfg(feature = "_regex")]
    #[arg(short = 'g', long, alias = "regexp", value_name = "REGEX", help = "Match regular expression")]
    regex: Option<String>,

    #[arg(short, long, value_parser = 0..=9, default_value = "3", value_name = "N", help = "Digits of precision")]
    digits: i64,

    #[arg(short, long, value_name = "N", help = "Limit output to top N values")]
    limit: Option<usize>,

    #[arg(short, long, value_name = "N", help = "Limit output to values seen at least N times")]
    min: Option<usize>,

    #[arg(short = 'x', long, value_name = "N", help = "Limit output to values seen at most N times")]
    max: Option<NonZeroUsize>,

    #[arg(short = 'I', long, conflicts_with = "lexigraphic", help = "Sort values with same frequency by original order [default]")]
    insertion: bool,

    #[arg(short = 'L', long, conflicts_with = "unstable", help = "Sort values with same frequency lexicographically")]
    lexigraphic: bool,

    #[arg(short = 'U', long, conflicts_with = "insertion", help = "Do not sort values with same frequency")]
    unstable: bool,

    #[arg(short = 'F', long, conflicts_with = "reverse", help = "Do not sort by frequency")]
    no_freq_sort: bool,

    #[arg(short = 'H', long, help = "Skip first line of each input file")]
    skip_header: bool,

    #[arg(short, long, conflicts_with = "no_freq_sort", help = "Output least common values first")]
    reverse: bool,

    #[arg(short = 'u', long, help = "Output unique values with no additional data")]
    unique: bool,

    #[arg(short, long, help = "Include line numbers")]
    number: bool,

    #[arg(short = 's', long, help = "Include running sum totals")]
    sum: bool,

    #[arg(short = 'P', long, help = "Omit percent column")]
    no_pct: bool,

    #[arg(short = 'C', long, help = "Omit CDF column")]
    no_cdf: bool,

    #[arg(short, long, conflicts_with = "csv", help = "Tab delimited output")]
    tsv: bool,

    #[arg(short, long, conflicts_with = "tsv", help = "Comma seperated output")]
    csv: bool,

    #[arg(short = 'V', long, alias = "semver", display_order = 1000, value_name = "RANGE", help = "Print version or check against semver range and exit")]
    version: Option<Option<String>>,

    #[arg(long = "feature", display_order = 1001, value_name = "FEATURE", help = "Check if compiled with feature and exit")]
    features: Option<Vec<String>>,

    files: Vec<String>,

    // files coming after `--`
    #[arg(last = true, allow_hyphen_values = true, hide = true)]
    files_raw: Vec<String>,

    #[arg(long = "", hide = true)]
    pub long_version: bool,
}
