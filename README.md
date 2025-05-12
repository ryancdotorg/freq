# freq

A tool for counting frequency of items and showing related statistics.

## Description

Over the years, I’ve found myself doing a lot of ad-hoc data analysis with
shell pipelines involving `grep`, `sed`, `awk`, `sort`, `uniq -c`, and `sort -rn`
to look at distributions of values in datasets. I wrote `freq` to streamline
these tasks, and I use it daily.

It has feature flags to enable transparent decompression of several file
types, and also regular expression filtering/munging support.

## Usage

```
Usage: freq [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -g, --regex <REGEX>   Match regular expression - behavior depends on capture groups.

                        * With no capture group, matching lines will be counted.
                        * With one capture group, the captured portion of matching
                          lines will be counted.
                        * With two named capture groups (`n` and `item`), `n`
                          will be parsed as the number of occurrences of `item`.

  -d, --digits <N>      Digits of precision [default: 3]
  -l, --limit <N>       Limit output to top N values
  -m, --min <N>         Limit output to values seen at least N times
  -x, --max <N>         Limit output to values seen at most N times
  -I, --insertion       Sort values with same frequency by original order [default]
  -L, --lexigraphic     Sort values with same frequency lexicographically
  -U, --unstable        Do not sort values with same frequency
  -F, --no-freq-sort    Do not sort by frequency
  -r, --reverse         Output least common values first
  -u, --unique          Output unique values with no additional data
  -n, --number          Include line numbers
  -s, --sum             Include running sum totals
  -P, --no-pct          Omit percent column
  -C, --no-cdf          Omit CDF column
  -t, --tsv             Tab delimited output
  -c, --csv             Comma seperated output
  -h, --help            Print help
  -V, --version         Print version
      --semver <RANGE>  Check version and exit
```

## Example Output

```
tr 'A-Z ' 'a-z\n' < 2600.txt.utf-8 | freq -l30 -g "^([a-z]+)[,.]*$"
  34269   6.547   6.547  the
  21801   4.165  10.712  and
  16573   3.166  13.879  to
  14950   2.856  16.735  of
  10394   1.986  18.721  a
   9574   1.829  20.550  he
   8848   1.690  22.240  in
   7949   1.519  23.759  his
   7587   1.450  25.209  that
   7302   1.395  26.604  was
   5669   1.083  27.687  with
   5348   1.022  28.709  had
   4656   0.890  29.598  it
   4587   0.876  30.474  her
   4578   0.875  31.349  not
   4503   0.860  32.209  at
   4294   0.820  33.030  him
   3913   0.748  33.777  as
   3879   0.741  34.519  on
   3663   0.700  35.218  but
   3427   0.655  35.873  for
   3316   0.634  36.507  she
   3264   0.624  37.130  i
   3163   0.604  37.735  is
   2994   0.572  38.307  you
   2757   0.527  38.833  said
   2670   0.510  39.343  from
   2640   0.504  39.848  all
   2401   0.459  40.306  were
   2390   0.457  40.763  by
```

## Getting Started

### Dependencies

* A Rust toolchain with `cargo`.

### Installation

Clone the repo, then

Basic: `cargo install --path freq --release`

All native Rust features: `cargo install --path freq --release --features ungz,unlz4,regex`

Full: `cargo install --path freq --release --features all`

## Help

You can file an issue on GitHub, however I may not respond. This software is
being provided without warranty in the hopes that it may be useful.

## Author

* [Ryan Castellucci](https://rya.nc/) [@ryancdotorg](https://github.com/ryancdotorg) https://rya.nc

## Donations

I am currently involved in a protracted
[civil rights case](https://www.leighday.co.uk/news/news/2023-news/legal-challenge-urges-government-to-give-legal-recognition-to-nonbinary-people/)
against the British government. If you find my work useful,
**please donate to my [crowdfunding effort](https://enby.org.uk/)**.

## License

This project may be used under the terms of your choice of the
[GNU GPL version 2](LICENSE.GPL-2.0), or the
[GNU GPL version 3](LICENSE.GPL-3.0).
