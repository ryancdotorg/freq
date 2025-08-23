// our code
use crate::input::Input;

#[cfg(feature = "egg")]
use crate::egg::egg;

use crate::ordered::OrderedString;

use crate::build_features::*;

// stdlib
use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{self, Write, LineWriter, BufRead};
use std::mem::take;
use std::num::{NonZeroUsize, NonZeroI32};

// packages
use clap::{Command, FromArgMatches, Parser};
use counter::Counter;
use semver::{Version, VersionReq};

#[cfg(all(feature = "regex-basic", not(feature = "regex-fancy")))]
use regex::{Regex, Captures};
#[cfg(feature = "regex-fancy")]
use fancy_regex::{Regex, Captures};
//#[cfg(all(feature = "regex-basic", feature = "regex-fancy"))]
//use regex::{Regex as RegexBasic, Captures as CapturesBasic};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/args.rs"));

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

type FnApplyRe<'a> = Box<dyn Fn(usize, &str) -> Option<(OrderedString, usize)> + 'a>;

#[cfg(feature = "_regex")]
//fn mk_apply_re(re: &Regex) -> Result<Box<dyn Fn(usize, &str) -> Option<(OrderedString, usize)> + '_>, FatalError> {
fn mk_apply_re(re: &Regex) -> Result<FnApplyRe<'_>, FatalError> {
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

    Ok(if has_n {
        if list.is_empty() {
            return Err(FatalError::ClapUnfmt(
                NonZeroI32::new(1).unwrap(),
                clap::error::Error::raw(
                    clap::error::ErrorKind::ValueValidation,
                    format!("Regex `{}` captures a count without a value", re),
                )
            ));
        }

        // return matched parts with count
        Box::new(move |i: usize, s: &str| {
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
                Some((OrderedString::new(i, item), n))
            } else {
                None
            }
        })
    } else if list.is_empty() {
        // return entire matched line
        Box::new(move |i: usize, s: &str| {
            if re_captures(re, s).is_some() {
                Some((OrderedString::new(i, s.to_string()), 1usize))
            } else {
                None
            }
        })
    } else {
        // return matched parts
        Box::new(move |i: usize, s: &str| {
            if let Some(captures) = re_captures(re, s) {
                let item = list.iter().map(|v| match v {
                        Group::Name(name) => captures.name(name),
                        Group::Number(num) => captures.get(*num),
                    })
                    .map(|v| v.map_or_else(|| "", |v| v.as_str()).to_string())
                    .collect::<Vec<_>>().join("\t");
                Some((OrderedString::new(i, item), 1usize))
            } else {
                None
            }
        })
    })
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
    // use 128 bit values to avoid overflows
    let (n, p_mod, div) = (n as u128, p_mod as u128, div as u128);
    // need + 5 for rounding
    let x = ((p_mod * n * 1000) / div + 5) / 10;
    ((x / p_mod).try_into().unwrap(), (x % p_mod).try_into().unwrap())
}

#[inline(always)]
fn pw_div(n: usize, div: usize) -> usize {
    // need + 5 for rounding
    ((n * 1000) / div + 5) / 10
}

pub enum FatalError {
    Misc(NonZeroI32, Box<dyn std::error::Error>),
    ClapFmt(NonZeroI32, clap::error::Error),
    ClapUnfmt(NonZeroI32, clap::error::Error),
}

impl FatalError {
    #[allow(dead_code)]
    pub fn new<E: Into<Box<dyn std::error::Error>>>(code: i32, err: E) -> Self {
        Self::Misc(code.try_into().unwrap_or(DEFAULT_ERROR_CODE), err.into())
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Misc(code, _) => code.get(),
            Self::ClapFmt(code, _) => code.get(),
            Self::ClapUnfmt(code, _) => code.get(),
        }
    }

    pub fn print(&self) {
        match self {
            Self::Misc(_, inner) => eprintln!("{}", inner),
            Self::ClapFmt(_, inner) => { let _ = inner.print(); },
            Self::ClapUnfmt(_, inner) => { let _ = inner.print(); },
        }
    }

    pub fn format(self, command: &mut Command) -> Self {
        match self {
            Self::ClapUnfmt(code, inner) => Self::ClapFmt(code, inner.format(command)),
            _ => self,
        }
    }
}

const DEFAULT_ERROR_CODE: NonZeroI32 = NonZeroI32::new(255).unwrap();

impl<E: Into<Box<dyn std::error::Error>>> From<E> for FatalError {
    fn from(e: E) -> Self {
        Self::Misc(DEFAULT_ERROR_CODE, e.into())
    }
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Misc(_, inner) => fmt::Display::fmt(inner, f),
            Self::ClapFmt(_, inner) => fmt::Display::fmt(inner, f),
            Self::ClapUnfmt(_, inner) => fmt::Display::fmt(inner, f),
        }
    }
}

impl fmt::Debug for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Misc(_, inner) => fmt::Debug::fmt(inner, f),
            Self::ClapFmt(_, inner) => fmt::Debug::fmt(inner, f),
            Self::ClapUnfmt(_, inner) => fmt::Debug::fmt(inner, f),
        }
    }
}

type CounterItem = (OrderedString, usize);
type FnCmp = fn(&CounterItem, &CounterItem) -> std::cmp::Ordering;

// wrapper around the args class
#[derive(Debug)]
pub(crate) struct Freq {
    pub args: FreqArgs,
    pub command: Command,
    pub long_version: bool,
}

impl Freq {
    pub fn from_command(command: Command) -> Result<Self, clap::error::Error> {
        let matches = command.clone().get_matches();
        let args = FreqArgs::from_arg_matches(&matches)?;

        Ok(Self {
            args,
            command,
            // HACK clap doesn't seem to have a way to differentiate long vs short flags...
            long_version: std::env::args().any(|arg| arg == "--version"),
        })
    }

    pub fn command(&self) -> Command {
        self.command.clone()
    }

    pub fn exec(mut self) -> Result<i32, FatalError> {
        let version_result = match self.args.version {
            Some(ref arg) => match arg {
                Some(_) => Some((self).check_version()?),
                None => {
                    let output = if self.long_version {
                        format!(
                            "{} {}\n",
                            self.command().get_name(),
                            get_long_version(),
                        )
                    } else {
                        self.command().render_version()
                    };

                    print!("{}", output);

                    return Ok(0);
                },
            },
            None => None,
        };

        let feature_result = match self.args.features {
            Some(ref features) => {
                let missing = features.iter()
                    .map(|f| f.to_ascii_uppercase())
                    .map(|f| f.replace("-", "_"))
                    .filter(|f| !FEATURES.contains(&f.as_str()))
                    .collect::<Vec<_>>();
                Some(if missing.is_empty() { 0 } else { 1 })
            },
            None => None,
        };

        if version_result.is_some() || feature_result.is_some() {
            if version_result == Some(1) || feature_result == Some(1) {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }

        self.check_args()?;

        let mut out: LineWriter<Box<dyn Write>> = if let Some(ref output) = self.args.output {
            LineWriter::new(Box::new(File::options()
                .write(true)
                .create(self.args.force)
                .create_new(!self.args.force)
                .open(output)?))
        } else {
            LineWriter::new(Box::new(io::stdout().lock()))
        };

        #[cfg(feature = "_regex")]
        let mut counter = if let Some(ref re) = self.args.regex {
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
        match (self.args.no_freq_sort, self.args.unstable) {
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

        let digits = usize::try_from(self.args.digits).unwrap();
        let lpad = !(self.args.tsv || self.args.csv);

        let mut parts = Vec::<Box<dyn Fn(usize, usize, usize, usize) -> String>>::new();

        // number lines
        if self.args.number {
            parts.push(mk_idx(max(6, 1 + n_width(distinct)), lpad));
        }

        parts.push(mk_cnt(max(7, 1 + n_width(most)), lpad));

        // running sum total
        if self.args.sum {
            let total = items.iter().fold(0, |accum, item| accum + item.1);
            parts.push(mk_run(max(7, 1 + n_width(total)), lpad));
        }

        // percent of total
        if !self.args.no_pct {
            parts.push(mk_pct(digits, lpad));
        }

        // cumulative distribution function
        if !self.args.no_cdf {
            parts.push(mk_cdf(digits, lpad));
        }

        // yay closures?
        let format_parts =
            move |i, c, r, t| parts.iter().map(|f| f(i, c, r, t)).collect::<Vec<String>>();

        // formatter (closures are, like, four layers deep at this point...)
        let f: Box<dyn Fn(usize, usize, usize, usize, String) -> String> = if self.args.unique {
            Box::new(move |_i, _c, _r, _t, v| v.to_string())
        } else if self.args.csv {
            // comma seperated
            Box::new(move |i, c, r, t, v| {
                let esc = v
                    .replace("\\", "\\\\")
                    .replace(",", "\\,")
                    .replace("\"", "\\\"");

                format!("{},\"{}\"", format_parts(i, c, r, t).join(","), esc)
            })
        } else if self.args.tsv {
            // tab delimited
            Box::new(move |i, c, r, t, v| format!("{}\t{}", format_parts(i, c, r, t).join("\t"), v))
        } else {
            // standard
            Box::new(move |i, c, r, t, v| format!("{}  {}", format_parts(i, c, r, t).join(""), v))
        };

        let limit = self.args.limit.unwrap_or(usize::MAX);

        for (index, count, value) in items
            .into_iter()
            .enumerate()
            .map(|(i, (v, c))| (i + 1, c, v))
        {
            if index > limit { break; }

            sum += count;

            if let Some(min) = self.args.min {
                if count < min {
                    continue;
                }
            }

            if let Some(max) = self.args.max {
                if count > max.into() {
                    continue;
                }
            }

            out.write_all(f(index, count, sum, total, value.into()).as_bytes())?;
            out.write_all(b"\n")?;
        }

        Ok(0)
    }

    fn check_version(&self) -> Result<i32, FatalError> {
        if let Some(semver) = self.args.version.as_ref().unwrap().as_ref() {
            let req = VersionReq::parse(semver)?;
            let ver = Version::parse(env!("CARGO_PKG_VERSION"))?;
            Ok(if req.matches(&ver) { 0 } else { 1 })
        } else {
            Ok(1)
        }
    }

    fn check_args(&self) -> Result<(), FatalError> {
        if let Some((min, max)) = self.args.min.zip(self.args.max) {
            if usize::from(max) < min {
                return Err(FatalError::ClapFmt(
                    NonZeroI32::new(1).unwrap(),
                    self.command().error(
                        clap::error::ErrorKind::ValueValidation,
                        "`max` can't be less than `min`",
                    )
                ));
            }
        }

        Ok(())
    }

    fn inputs(&mut self) -> Result<Vec<Input<'_>>, FatalError> {
        // open input files, triggering i/o errors
        let inputs = take(&mut self.args.files).into_iter()
            .map(|f| if f == "-" { None } else { Some(f) })
            .chain(take(&mut self.args.files_raw).into_iter().map(Some))
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
        self.counter_call(&|i, s| Some((OrderedString::new(i, s.to_string()), 1usize)))
    }

    #[cfg(feature = "_regex")]
    fn counter_regex(&mut self, re: &Regex) -> Result<Counter<OrderedString>, FatalError> {
        // create closure to apply regular expression
        let apply_re = mk_apply_re(re);
        match apply_re {
            Ok(ref apply_re) => self.counter_call(apply_re),
            Err(e) => Err(e.format(&mut self.command())),
        }
    }

    #[allow(dead_code)]
    fn counter_call<F: Fn(usize, &str) -> Option<(OrderedString, usize)>>(&mut self, f: &F) -> Result<Counter<OrderedString>, FatalError> {
        let skip = if self.args.skip_header { 1 } else { 0 };
        // run the counter over the lines
        Ok(self.inputs()?
            .into_iter()
            .flat_map(|i| {
                let label = i.get_label().to_string();
                i.lines()
                    .enumerate()
                    .skip(skip)
                    .filter_map(move |(index, line)| {
                        match line {
                            Err(e) => {
                                eprintln!(
                                    "{}:{}:Error({}): {}",
                                    label, index, e.kind(), e,
                                );
                                None
                            },
                            Ok(s) => f(index, &s),
                        }
                    })
            })
            .collect::<Counter<_>>())
    }

    fn cmp_freq(&self) -> FnCmp {
        // sort ascending or descending depending on flag
        if self.args.reverse {
            |(_, a), (_, b)| a.cmp(b)
        } else {
            |(_, a), (_, b)| b.cmp(a)
        }
    }

    fn cmp_str(&self) -> FnCmp {
        // sort by lexigraphic or insertion order depending on flag
        if self.args.lexigraphic {
            |(a, _), (b, _)| a.as_ref().cmp(b.as_ref())
        } else {
            |(a, _), (b, _)| a.cmp(b)
        }
    }
}
