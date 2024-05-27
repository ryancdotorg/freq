// our code
mod input;
use input::Input;

// stdlib
use std::io::BufRead;
use std::process::exit;

// packages
use clap::Parser;
use counter::Counter;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, help = "Number lines")]
    number: bool,

    #[arg(short, help = "Show percent")]
    percent: bool,

    #[arg(short, help = "Show CDF")]
    cdf: bool,

    #[arg(short, value_parser = 0..=8, default_value = "3", help = "Digits of precision")]
    digits: i64,

    #[arg(short = 'T', conflicts_with = "csv", help = "Tab-delimited output")]
    tab: bool,

    #[arg(short = 'C', conflicts_with = "tab", help = "CSV output")]
    csv: bool,

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

// let (cdf_whole, cdf_frac) = pf_div(accumulated, p_mod, total);
// index, count, accumulated, total
//"{:w1$}  {:w2$} {:3}.{:0>w3$} {:3}.{:0>w3$}  {}",

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

#[inline(always)]

fn main() {
    let cli = Cli::parse();

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
                if input.is_err() {
                    let e = input.err().unwrap();
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
                if line.is_err() {
                    let e = line.err().unwrap();
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

    let mut accumulated = 0;
    let most = items[0].1;

    let digits = usize::try_from(cli.digits).unwrap();

    // n_width(distinct)
    // n_width(most)

    let lpad = !(cli.tab || cli.csv);

    let f_idx = mk_idx(1 + n_width(distinct), lpad);
    let f_cnt = mk_cnt(1 + n_width(most), lpad);
    let f_pct = mk_pct(digits, lpad);
    let f_cdf = mk_cdf(digits, lpad);

    let mut parts = Vec::<Box<dyn Fn(usize, usize, usize, usize) -> String>>::new();

    if cli.number {
        parts.push(f_idx);
    }
    parts.push(f_cnt);
    if cli.percent {
        parts.push(f_pct);
    }
    if cli.cdf {
        parts.push(f_cdf);
    }

    // formatter
    let f: Box<dyn Fn(usize, usize, usize, usize, String) -> String> = if cli.tab {
        Box::new(move |i, c, a, t, v| {
            format!(
                "{}\t{}",
                parts
                    .iter()
                    .map(|f| f(i, c, a, t))
                    .collect::<Vec<String>>()
                    .join("\t"),
                v,
            )
        })
    } else if cli.csv {
        Box::new(move |i, c, a, t, v| {
            let esc = v
                .replace("\\", "\\\\")
                .replace(",", "\\,")
                .replace("\"", "\\\"");

            format!(
                "{},\"{}\"",
                parts
                    .iter()
                    .map(|f| f(i, c, a, t))
                    .collect::<Vec<String>>()
                    .join(","),
                esc,
            )
        })
    } else {
        Box::new(move |i, c, a, t, v| {
            format!(
                "{}  {}",
                parts
                    .iter()
                    .map(|f| f(i, c, a, t))
                    .collect::<Vec<String>>()
                    .join(""),
                v,
            )
        })
    };

    for (index, count, value) in items
        .into_iter()
        .enumerate()
        .map(|(i, (v, c))| (i + 1, c, v))
    {
        accumulated += count;

        println!("{}", f(index, count, accumulated, total, value));
    }
}
