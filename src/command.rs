// our code
use crate::input::Input;

#[cfg(feature = "egg")]
use crate::egg::egg;

use crate::ordered::OrderedString;

use crate::build_features::*;

// stdlib
use std::cmp::max;
use std::fmt;
use std::io::{self, BufRead};
use std::mem::take;
use std::num::{NonZeroUsize, NonZeroI32};
use std::ops::Deref;

// packages
//use clap::builder::styling::*;
use clap::{CommandFactory, Parser};
use counter::Counter;
use semver::{Version, VersionReq};

#[cfg(all(feature = "regex-basic", not(feature = "regex-fancy")))]
use regex::{Regex, Captures};
#[cfg(feature = "regex-fancy")]
use fancy_regex::{Regex, Captures};

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author, about, long_about = None)]
pub struct Freq {
    #[cfg(feature = "_regex")]
    #[arg(short = 'g', long, alias = "regexp", value_name = "REGEX", help = "Match regular expression (--regex-help for details)")]
    regex: Option<String>,

    #[arg(short, long, value_parser = 0..=8, default_value = "3", value_name = "N", help = "Digits of precision")]
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

    #[arg(long = "check-version", alias = "semver", display_order = 1000, value_name = "RANGE", help = "Check version against a semver range and exit")]
    semver: Option<String>,

    #[arg(long, display_order = 1001, value_name = "FEATURE", help = "Check if compiled with specified feature and exit")]
    check_feature: Option<Vec<String>>,

    files: Vec<String>,

    // files coming after `--`
    #[arg(last = true, allow_hyphen_values = true, hide = true)]
    files_raw: Vec<String>,
}

#[cfg(all(feature = "regex-basic", not(feature = "regex-fancy")))]
#[inline]
fn re_captures<'a>(re: &'a Regex, s: &'a str) -> Option<Captures<'a>> {
    re.captures(s)
}

#[cfg(feature = "regex-fancy")]
#[inline]
fn re_captures<'a>(re: &'a Regex, s: &'a str) -> Option<Captures<'a>> {
    re.captures(s).unwrap_or(None)
}

#[cfg(feature = "_regex")]
fn mk_apply_re(re: &Regex) -> Box<dyn Fn(&str) -> Option<(usize, String)> + '_> {
    use std::collections::HashSet;
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    enum Group {
        Name(String),
        Number(usize),
    }

    let mut data = re.capture_names()
        .enumerate()
        .skip(1)
        .map(|(a, b)| b.map_or_else(
            || Group::Number(a),
            |v| Group::Name(v.into()))
        )
        .collect::<HashSet<_>>();

    let has_n = data.remove(&Group::Name("n".into()));

    let mut list = data.into_iter().collect::<Vec<_>>();
    list.sort();

    if has_n {
        if list.is_empty() {
            panic!("no data capture");
        }

        // return matched parts with count
        Box::new(move |s: &str| {
            if let Some(captures) = re_captures(re, s) {
                let n: usize = captures.name("n")
                    .expect("no group n")
                    .as_str()
                    .parse()
                    .expect("group n doesn't contain a number");
                let item = list.iter().map(|v| match v {
                        Group::Name(name) => captures.name(name),
                        Group::Number(num) => captures.get(*num),
                    })
                    .map(|v| v.map_or_else(|| "", |v| v.as_str()).to_string())
                    .collect::<Vec<_>>().join("\t");
                Some((n, item))
            } else {
                None
            }
        })
    } else if list.is_empty() {
        // return entire matched line
        Box::new(move |s: &str| {
            if let Some(_) = re_captures(re, s) {
                Some((1usize, s.to_string()))
            } else {
                None
            }
        })
    } else {
        // return matched parts
        Box::new(move |s: &str| {
            if let Some(captures) = re_captures(re, s) {
                let item = list.iter().map(|v| match v {
                        Group::Name(name) => captures.name(name),
                        Group::Number(num) => captures.get(*num),
                    })
                    .map(|v| v.map_or_else(|| "", |v| v.as_str()).to_string())
                    .collect::<Vec<_>>().join("\t");
                Some((1usize, item))
            } else {
                None
            }
        })
    }
}

fn n_width(n: usize) -> usize {
    match n {
        0 => 1,
        _ => (n.ilog10() + 1).try_into().unwrap(),
    }
}

fn mk_fmt_pct(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize) -> String> {
    let p_mod = 10_usize.pow(digits.try_into().unwrap());
    if lpad && digits > 0 {
        Box::new(move |n, t| {
            let (whole, frac) = pf_div(n, p_mod, t);
            format!("{:4}.{:0>digits$}", whole, frac)
        })
    } else if digits > 0 {
        Box::new(move |n, t| {
            let (whole, frac) = pf_div(n, p_mod, t);
            format!("{}.{:0>digits$}", whole, frac)
        })
    } else if lpad {
        Box::new(move |n, t| format!("{:4}", pw_div(n, t)))
    } else {
        Box::new(move |n, t| pw_div(n, t).to_string())
    }
}

fn mk_fmt_int(digits: usize, lpad: bool) -> Box<dyn Fn(usize) -> String> {
    if lpad && digits > 1 {
        Box::new(move |n| format!("{:>digits$}", n))
    } else {
        Box::new(|n| n.to_string())
    }
}

fn mk_idx(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_int(digits, lpad);
    Box::new(move |i, _c, _r, _t| f(i))
}

fn mk_cnt(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_int(digits, lpad);
    Box::new(move |_i, c, _r, _t| f(c))
}

fn mk_run(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_int(digits, lpad);
    Box::new(move |_i, _c, r, _t| f(r))
}

fn mk_pct(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_pct(digits, lpad);
    Box::new(move |_i, c, _r, t| f(c, t))
}

fn mk_cdf(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_pct(digits, lpad);
    Box::new(move |_i, _c, r, t| f(r, t))
}

#[inline(always)]
fn pf_div(n: usize, p_mod: usize, div: usize) -> (usize, usize) {
    // need + 5 for rounding
    let x = ((p_mod * n * 1000) / div + 5) / 10;
    (x / p_mod, x % p_mod)
}

#[inline(always)]
fn pw_div(n: usize, div: usize) -> usize {
    // need + 5 for rounding
    ((n * 1000) / div + 5) / 10
}

#[derive(Debug)]
pub struct FatalError(NonZeroI32, Box<dyn std::error::Error>);

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.1.fmt(f) }
}

const DEFAULT_ERROR_CODE: NonZeroI32 = NonZeroI32::new(255).unwrap();

impl FatalError {
    #[allow(dead_code)]
    pub fn new<E: Into<Box<dyn std::error::Error>>>(code: i32, err: E) -> Self {
        Self(code.try_into().unwrap_or(DEFAULT_ERROR_CODE), err.into())
    }

    pub fn exit_code(&self) -> i32 {
        self.0.get()
    }
}

impl<E: Into<Box<dyn std::error::Error>>> From<E> for FatalError {
    fn from(e: E) -> Self {
        Self(DEFAULT_ERROR_CODE, e.into())
    }
}

impl Deref for FatalError {
    type Target = Box<dyn std::error::Error>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

type CounterItem = (OrderedString, usize);
type FnCmp = fn(&CounterItem, &CounterItem) -> std::cmp::Ordering;

impl Freq {
    pub fn exec(mut self) -> Result<i32, FatalError> {
        let version_result = match self.semver {
            Some(_) => Some((&self).check_version()?),
            None => None,
        };

        if let Some(version_result) = version_result {
            return Ok(version_result);
        }

        if let Some(check_feature) = self.check_feature {
            let missing = check_feature.iter()
                .map(|f| f.to_ascii_uppercase())
                .map(|f| f.replace("-", "_"))
                .filter(|f| !FEATURES.contains(&f.as_str()))
                .collect::<Vec<_>>();
            println!("{:?}", missing);
            return Ok(0);
        }

        self.check_args()?;

        #[cfg(feature = "_regex")]
        let mut counter = if let Some(ref re) = self.regex {
            self.counter_regex(&Regex::new(re)?)?
        } else {
            self.counter()?
        };

        #[cfg(not(feature = "_regex"))]
        let mut counter = self.counter()?;

        // return success if there's no data
        if counter.is_empty() {
            return Ok(0);
        }

        let distinct = counter.len();
        let total = counter.total::<usize>();

        // drain/collect instead of Counter::most_common_ordered saves memory
        let mut items: Vec<CounterItem> = counter.drain().collect();

        // sort according to options
        match (self.no_freq_sort, self.unstable) {
            (false, true) => { // sort by frequency only
                items.sort_unstable_by(self.cmp_freq());
            },
            (true, false) => { // sort by string only
                items.sort_unstable_by(self.cmp_str());
            },
            (false, false) => { // sort by frequency, then string
                items.sort_unstable_by(|a, b| self.cmp_freq()(a, b).then_with(|| self.cmp_str()(a, b)));
            },
            (true, true) => (), // don't sort at all
        }

        let mut sum = 0;
        let most = items[0].1;

        let digits = usize::try_from(self.digits).unwrap();
        let lpad = !(self.tsv || self.csv);

        let mut parts = Vec::<Box<dyn Fn(usize, usize, usize, usize) -> String>>::new();

        // number lines
        if self.number {
            parts.push(mk_idx(max(6, 1 + n_width(distinct)), lpad));
        }

        parts.push(mk_cnt(max(7, 1 + n_width(most)), lpad));

        // running sum total
        if self.sum {
            let total = items.iter().fold(0, |accum, item| accum + item.1);
            parts.push(mk_run(max(7, 1 + n_width(total)), lpad));
        }

        // percent of total
        if !self.no_pct {
            parts.push(mk_pct(digits, lpad));
        }

        // cumulative distribution function
        if !self.no_cdf {
            parts.push(mk_cdf(digits, lpad));
        }

        // yay closures?
        let format_parts =
            move |i, c, r, t| parts.iter().map(|f| f(i, c, r, t)).collect::<Vec<String>>();

        // formatter (closures are, like, four layers deep at this point...)
        let f: Box<dyn Fn(usize, usize, usize, usize, String) -> String> = if self.unique {
            Box::new(move |_i, _c, _r, _t, v| v.to_string())
        } else if self.csv {
            // comma seperated
            Box::new(move |i, c, r, t, v| {
                let esc = v
                    .replace("\\", "\\\\")
                    .replace(",", "\\,")
                    .replace("\"", "\\\"");

                format!("{},\"{}\"", format_parts(i, c, r, t).join(","), esc)
            })
        } else if self.tsv {
            // tab delimited
            Box::new(move |i, c, r, t, v| format!("{}\t{}", format_parts(i, c, r, t).join("\t"), v))
        } else {
            // standard
            Box::new(move |i, c, r, t, v| format!("{}  {}", format_parts(i, c, r, t).join(""), v))
        };

        let limit = self.limit.unwrap_or(usize::MAX);

        for (index, count, value) in items
            .into_iter()
            .enumerate()
            .map(|(i, (v, c))| (i + 1, c, v))
        {
            if index > limit { break; }

            sum += count;

            if let Some(min) = self.min {
                if count < min {
                    continue;
                }
            }

            if let Some(max) = self.max {
                if count > max.into() {
                    continue;
                }
            }

            println!("{}", f(index, count, sum, total, value.into()));
        }

        Ok(0)
    }

    fn check_version(&self) -> Result<i32, FatalError> {
        let req = VersionReq::parse(self.semver.as_ref().unwrap())?;
        let ver = Version::parse(env!("CARGO_PKG_VERSION"))?;
        Ok(if req.matches(&ver) { 0 } else { 1 })
    }

    fn check_args(&self) -> Result<(), FatalError> {
        if let Some((min, max)) = self.min.zip(self.max) {
            if usize::from(max) < min {
                return Err(Self::command().error(
                    clap::error::ErrorKind::ArgumentConflict,
                    "`max` can't be less than `min`",
                ).into());
            }
        }

        Ok(())
    }

    fn inputs(&mut self) -> Result<Vec<Input>, FatalError> {
        // open input files, triggering i/o errors
        let inputs = take(&mut self.files).into_iter()
            .map(|f| if f == "-" { None } else { Some(f) })
            .chain(take(&mut self.files_raw).into_iter().map(Some))
            .map(|f| match f {
                Some(f) => match Input::path(&f) {
                    Ok(input) => Ok(input),
                    Err(e) => {
                        #[cfg(feature = "egg")]
                        if f == "out" { egg(); }
                        Err(io::Error::new(
                                e.kind(),
                                format!("Error opening `{}`: {}", f, e),
                        ))
                    },
                },
                None => Input::stdin(),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if inputs.is_empty() {
            Ok(vec![Input::stdin()?])
        } else {
            Ok(inputs)
        }
    }

    fn counter(&mut self) -> Result<Counter<OrderedString>, FatalError> {
        // run the counter over the lines
        Ok(self.inputs()?
            .into_iter()
            .flat_map(|i| {
                let label = i.get_label().to_string();
                i.lines().enumerate().filter_map(move |(index, line)| {
                    match line {
                        Err(e) => {
                            eprintln!("{}:{}:Error({}): {}", label, index, e.kind(), e,);
                            None
                        },
                        Ok(s) => Some(OrderedString::new(index, s)),
                    }
                })
            })
            .collect::<Counter<_>>())
    }

    #[cfg(feature = "_regex")]
    fn counter_regex(&mut self, re: &Regex) -> Result<Counter<OrderedString>, FatalError> {
        // create closure to apply regular expression
        let ref apply_re = mk_apply_re(re);

        // run the counter over the lines
        Ok(self.inputs()?
            .into_iter()
            .flat_map(|i| {
                let label = i.get_label().to_string();
                i.lines().enumerate().filter_map(move |(index, line)| {
                    match line {
                        Err(e) => {
                            eprintln!("{}:{}:Error({}): {}", label, index, e.kind(), e,);
                            None
                        },
                        Ok(s) => {
                            apply_re(&s).map(|(count, item)| (OrderedString::new(index, item), count))
                        }
                    }
                })
            })
            .collect::<Counter<_>>())
    }

    fn cmp_freq(&self) -> FnCmp {
        // sort ascending or descending depending on flag
        if self.reverse {
            |(_, a), (_, b)| a.cmp(b)
        } else {
            |(_, a), (_, b)| b.cmp(a)
        }
    }

    fn cmp_str(&self) -> FnCmp {
        // sort by lexigraphic or insertion order depending on flag
        if self.lexigraphic {
            |(a, _), (b, _)| a.as_ref().cmp(b.as_ref())
        } else {
            |(a, _), (b, _)| a.cmp(b)
        }
    }
}
