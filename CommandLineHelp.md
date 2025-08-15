# Command-Line Help for `freq`

This document contains the help content for the `freq` command-line program.

**Command Overview:**

* [`freq`↴](#freq)

## `freq`

A command line tool for counting frequency of items and showing related statistics.

**Usage:** `freq [OPTIONS] [FILES]...`

###### **Arguments:**

* `<FILES>`
* `<FILES_RAW>`

###### **Options:**

* `-o`, `--output <FILE>` — Write output to FILE. If this is not specified, output will be sent to STDOUT.
* `-f`, `--force` — By default, when an output file is specified with `-o`, `freq` will not overwrite files that already exist. Pass `-f` to override this precaution.
* `-g`, `--regex <REGEX>` — Match regular expression. With no capture groups, this will act as a filter. With a capture groups, the text within the capture groups is joined with tabs to become the value. Named capture groups are sorted lexically using the names as keys. Unnamed capture groups come after named capture groups.

   The capture group named `n` is interpreted as the number of times a value appears, if used you will also need to capture a value. This is useful to reprocess previous output of `freq`.
* `-d`, `--digits <N>` — Specify how many decimal places to use when printing percentages. Valid values are 0 to 9.

  Default value: `3`
* `-l`, `--limit <N>` — Limit output to top N values
* `-m`, `--min <N>` — Limit output to values seen at least N times
* `-x`, `--max <N>` — Limit output to values seen at most N times
* `-I`, `--insertion` — Sort values with the same frequency in the order in which they were originally seen. Enabled by default unless another sort option is set.
* `-L`, `--lexigraphic` — Sort values with same frequency lexicographically
* `-U`, `--unstable` — Do not sort values with same frequency
* `-F`, `--no-freq-sort` — Do not sort by frequency
* `-H`, `--skip-header` — Skip first line of each input file
* `-r`, `--reverse` — Output least common values first
* `-u`, `--unique` — Output unique values with no additional data
* `-n`, `--number` — Include line numbers
* `-s`, `--sum` — Include running sum totals
* `-P`, `--no-pct` — Omit percent column
* `-C`, `--no-cdf` — Omit CDF column
* `-t`, `--tsv` — Tab delimited output
* `-c`, `--csv` — Comma seperated output
* `-V`, `--version <RANGE>` — Print version or check against semver range and exit
* `--feature <FEATURE>` — Check if compiled with feature and exit



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
