#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author, about, long_about = None)]
pub(crate) struct FreqArgs {
    #[arg(
        short, long, value_name = "FILE",
        help = "Write output to FILE [default: STDOUT]",
        long_help = "Write output to FILE. If this is not specified, output will be sent to STDOUT.",
    )]
    output: Option<String>,

    #[arg(
        short, long,
        help = "Allow overwriting existing files with -o or --output",
        long_help = "By default, when an output file is specified with `-o`, `freq` will not overwrite files that already exist. Pass `-f` to override this precaution.",
    )]
    force: bool,

    #[cfg(feature = "_regex")]
    #[arg(
        short = 'g', long, alias = "regexp", value_name = "REGEX",
        help = "Match regular expression",
        long_help = "Match regular expression. With no capture groups, this will act as a filter. With capture groups, the text within the capture groups is joined with tabs to become the value. Named capture groups are sorted lexically using the names as keys. Unnamed capture groups come after named capture groups.\n\nThe capture group named `n` is interpreted as the number of times a value appears. If used, you will also need to capture a value. This is useful to reprocess previous output of `freq`.",
    )]
    regex: Option<String>,

    #[arg(
        short, long, value_parser = 0..=9, default_value = "3", value_name = "N",
        help = "Digits of precision",
        long_help = "Specify how many decimal places to use when printing percentages. Valid values are 0 to 9.",
    )]
    digits: i64,

    #[arg(
        short, long, value_name = "N",
        help = "Limit output to top N values",
    )]
    limit: Option<usize>,

    #[arg(
        short, long, value_name = "N",
        help = "Limit output to values seen at least N times",
    )]
    min: Option<usize>,

    #[arg(
        short = 'x', long, value_name = "N",
        help = "Limit output to values seen at most N times",
    )]
    max: Option<NonZeroUsize>,

    #[arg(
        short = 'I', long, conflicts_with = "lexigraphic",
        help = "Sort values with same frequency by original order [default]",
        long_help = "Sort values with the same frequency in the order in which they were originally seen. Enabled by default unless another sort option is set.",
    )]
    insertion: bool,

    #[arg(
        short = 'L', long, conflicts_with = "unstable",
        help = "Sort values with same frequency lexicographically",
    )]
    lexigraphic: bool,

    #[arg(
        short = 'U', long, conflicts_with = "insertion",
        help = "Do not sort values with same frequency",
    )]
    unstable: bool,

    #[arg(
        short = 'F', long, conflicts_with = "reverse",
        help = "Do not sort by frequency",
    )]
    no_freq_sort: bool,

    #[arg(
        short = 'H', long,
        help = "Skip first line of each input file",
    )]
    skip_header: bool,

    #[arg(
        short, long, conflicts_with = "no_freq_sort",
        help = "Output least common values first",
    )]
    reverse: bool,

    #[arg(
        short, long,
        help = "Output unique values with no additional data",
    )]
    unique: bool,

    #[arg(
        short, long,
        help = "Include line numbers",
    )]
    number: bool,

    #[arg(
        short, long,
        help = "Include running sum totals",
    )]
    sum: bool,

    #[arg(
        short = 'P', long,
        help = "Omit percent column",
    )]
    no_pct: bool,

    #[arg(
        short = 'C', long,
        help = "Omit CDF column",
    )]
    no_cdf: bool,

    #[arg(
        short, long, conflicts_with = "csv",
        help = "Tab delimited output",
    )]
    tsv: bool,

    #[arg(
        short, long, conflicts_with = "tsv",
        help = "Comma seperated output",
    )]
    csv: bool,

    #[arg(
        short = 'V', long, alias = "semver", display_order = 1000, value_name = "RANGE",
        help = "Print version or check against semver range and exit",
    )]
    version: Option<Option<String>>,

    #[arg(
        long = "feature", display_order = 1001, value_name = "FEATURE", hide_short_help = true,
        help = "Check if compiled with feature and exit",
    )]
    features: Option<Vec<String>>,

    files: Vec<String>,

    // files coming after `--`
    #[arg(last = true, allow_hyphen_values = true, hide = true)]
    files_raw: Vec<String>,
}
