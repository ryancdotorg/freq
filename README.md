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

TODO: better regex documentation

```
Usage: freq [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -o, --output <FILE>      Write output to FILE [default: STDOUT]
  -g, --regex <REGEX>      Match regular expression
  -d, --digits <N>         Digits of precision [default: 3]
  -l, --limit <N>          Limit output to top N values
  -m, --min <N>            Limit output to values seen at least N times
  -x, --max <N>            Limit output to values seen at most N times
  -I, --insertion          Sort values with same frequency by original order [default]
  -L, --lexigraphic        Sort values with same frequency lexicographically
  -U, --unstable           Do not sort values with same frequency
  -F, --no-freq-sort       Do not sort by frequency
  -H, --skip-header        Skip first line of each input file
  -r, --reverse            Output least common values first
  -u, --unique             Output unique values with no additional data
  -n, --number             Include line numbers
  -s, --sum                Include running sum totals
  -P, --no-pct             Omit percent column
  -C, --no-cdf             Omit CDF column
  -t, --tsv                Tab delimited output
  -c, --csv                Comma seperated output
  -h, --help               Print help
  -V, --version            Print version
  -V, --version [<RANGE>]  Print version or check against semver range and exit
      --feature <FEATURE>  Check if compiled with feature and exit
```

## Example Outputs

### Word Count
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

### IP Addresses with Most Distinct User Agent Strings
```
freq -FUug '\S+\s+(\S+)\s+(?:\S+\s+){10}"([^"]+)"' /var/log/nginx/access.log | freq -l25 -Lng '^(\S+)'
     1     94   1.097   1.097  141.95.205.46
     2     75   0.875   1.972  57.128.95.174
     3     73   0.852   2.823  141.94.131.5
     4     68   0.793   3.616  162.19.29.212
     5     68   0.793   4.410  51.210.99.95
     6     68   0.793   5.203  57.128.95.175
     7     67   0.782   5.985  135.125.104.28
     8     67   0.782   6.766  141.95.205.41
     9     63   0.735   7.501  57.128.118.108
    10     63   0.735   8.236  57.128.95.181
    11     61   0.712   8.948  57.128.119.15
    12     60   0.700   9.648  57.128.95.182
    13     59   0.688  10.336  162.19.87.99
    14     58   0.677  11.013  57.128.118.171
    15     54   0.630  11.643  57.128.95.173
    16     47   0.548  12.191  57.128.118.175
    17     31   0.362  12.552  141.95.205.35
    18     30   0.350  12.902  78.153.140.177
    19     26   0.303  13.206  2a01:4f8:222:114d::2
    20     23   0.268  13.474  2a01:4f8:272:5d4a::2
    21     18   0.210  13.684  23.88.72.209
    22     17   0.198  13.882  2a01:4f8:140:9402::2
    23     17   0.198  14.081  5.75.246.132
    24     15   0.175  14.256  146.255.56.82
    25     15   0.175  14.431  45.144.212.129
```

## Getting Started

### Dependencies

* A Rust toolchain with `cargo`.

### Installation

Clone the repo, then

Lite: `cargo install --path freq --no-default-features`

Standard: `cargo install --path freq`

Full: `cargo install --path freq --features full`

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
