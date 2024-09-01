// our code
mod input;
use input::Input;

mod egg;
use egg::egg;

// stdlib
use std::cmp::max;
use std::io::{self, BufRead, Write};
use std::process::exit;

// packages
use clap::{CommandFactory, FromArgMatches, Parser};
use counter::Counter;

build_info::build_info!(fn binfo);

fn get_long_version() -> &'static str {
    let info = binfo();
    let mut parts = Vec::<String>::new();
    parts.push("v".to_string());
    parts.push(info.crate_info.version.to_string());

    if let Some(vc) = &info.version_control {
        if let Some(git) = &vc.git() {
            parts.push("+".to_string());
            if let Some(branch) = &git.branch {
                parts.push(format!("{}.", branch));
            }
            parts.push(git.commit_short_id.to_string());
            if git.dirty {
                parts.push("-dirty".to_string());
            }
        }
    }

    parts.push(build_info::format!(
        "\nBuilt at {} with {}",
        $.timestamp,
        $.compiler,
    ).to_string());

    Box::leak(parts.join("").into_boxed_str())
}

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = build_info::format!("v{}", $.crate_info.version))]
#[command(author, about, long_about = None)]
struct Cli {
    #[arg(short, value_parser = 0..=8, default_value = "3", help = "Digits of precision")]
    digits: i64,

    #[arg(short, help = "Limit output to top N values")]
    limit: Option<usize>,

    #[arg(short, long, help = "Number lines")]
    number: bool,

    #[arg(short = 'U', long, help = "Just output unique lines")]
    uniq: bool,

    #[arg(short, long, conflicts_with = "csv", help = "Tab delimited output")]
    tsv: bool,

    #[arg(short, long, conflicts_with = "tsv", help = "Comma seperated output")]
    csv: bool,

    #[arg(short = 'P', long, help = "Don't show percent")]
    no_pct: bool,

    #[arg(short = 'C', long, help = "Don't show CDF")]
    no_cdf: bool,

    files: Vec<String>,

    #[arg(last = true, allow_hyphen_values = true, hide = true)]
    files_raw: Vec<String>,
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
    Box::new(move |i, _c, _a, _t| f(i))
}

fn mk_cnt(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_int(digits, lpad);
    Box::new(move |_i, c, _a, _t| f(c))
}

fn mk_pct(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_pct(digits, lpad);
    Box::new(move |_i, c, _a, t| f(c, t))
}

fn mk_cdf(digits: usize, lpad: bool) -> Box<dyn Fn(usize, usize, usize, usize) -> String> {
    let f = mk_fmt_pct(digits, lpad);
    Box::new(move |_i, _c, a, t| f(a, t))
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

fn main() {
    let command = Cli::command();
    let cli = Cli::from_arg_matches(&command
                                   .long_version(get_long_version())
                                   .get_matches()
                                   ).unwrap();

    // open the input files, triggering i/o errors
    let inputs: Vec<Input> = if cli.files.len() + cli.files_raw.len() > 0 {
        cli.files
            .iter()
            .map(|f| match f.as_str() {
                "-" => (f, Input::stdin()),
                _ => (f, Input::path(f)),
            })
            .chain(cli.files_raw.iter().map(|f| (f, Input::path(f))))
            .map(|(f, input)| {
                if let Err(e) = input {
                    if f == "out" { egg(); }
                    eprintln!("Error opening `{}`: {}", f, e);
                    exit(1);
                }
                input.unwrap()
            })
            .collect()
    } else {
        vec![Input::stdin().unwrap()]
    };

    // run the counter over the lines
    let mut counter = inputs
        .into_iter()
        .flat_map(|i| {
            let label = i.get_label();
            i.lines().enumerate().filter_map(move |(index, line)| {
                if let Err(e) = line {
                    eprintln!("{}:{}:Error({}): {}", label, index, e.kind(), e,);
                    None
                } else {
                    line.ok()
                }
            })
        })
        .collect::<Counter<_>>();

    let distinct = counter.len();
    let total = counter.total::<usize>();

    // drain/collect instead of Counter::most_common_ordered saves memory
    let mut items = counter.drain().collect::<Vec<_>>();
    items.sort_unstable_by(|(a_value, a_count), (b_value, b_count)| {
        b_count.cmp(a_count).then_with(|| a_value.cmp(b_value))
    });

    if items.len() == 0 {
        exit(0);
    }

    let mut accumulated = 0;
    let most = items[0].1;

    let digits = usize::try_from(cli.digits).unwrap();
    let lpad = !(cli.tsv || cli.csv);

    let mut parts = Vec::<Box<dyn Fn(usize, usize, usize, usize) -> String>>::new();

    // number lines
    if cli.number {
        parts.push(mk_idx(max(6, 1 + n_width(distinct)), lpad));
    }

    parts.push(mk_cnt(max(7, 1 + n_width(most)), lpad));

    // percent of total
    if !cli.no_pct {
        parts.push(mk_pct(digits, lpad));
    }

    // cumulative distribution function
    if !cli.no_cdf {
        parts.push(mk_cdf(digits, lpad));
    }

    // yay closures?
    let format_parts =
        move |i, c, a, t| parts.iter().map(|f| f(i, c, a, t)).collect::<Vec<String>>();

    // formatter (closures are, like, four layers deep at this point...)
    let f: Box<dyn Fn(usize, usize, usize, usize, String) -> String> = if cli.uniq {
        Box::new(move |_i, _c, _a, _t, v| format!("{}", v))
    } else if cli.csv {
        // comma seperated
        Box::new(move |i, c, a, t, v| {
            let esc = v
                .replace("\\", "\\\\")
                .replace(",", "\\,")
                .replace("\"", "\\\"");

            format!("{},\"{}\"", format_parts(i, c, a, t).join(","), esc)
        })
    } else if cli.tsv {
        // tab delimited
        Box::new(move |i, c, a, t, v| format!("{}\t{}", format_parts(i, c, a, t).join("\t"), v))
    } else {
        // standard
        Box::new(move |i, c, a, t, v| format!("{}  {}", format_parts(i, c, a, t).join(""), v))
    };

    let mut stdout = io::stdout();
    let limit = cli.limit.unwrap_or(usize::MAX);

    for (index, count, value) in items
        .into_iter()
        .enumerate()
        .map(|(i, (v, c))| (i + 1, c, v))
    {
        if index > limit { break; }

        accumulated += count;

        let _ = writeln!(stdout, "{}", f(index, count, accumulated, total, value));
    }
}
