use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

// Expose the test macro to the entire crate
#[macro_use]
mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Debug, Parser)]
struct Options {
    /// The day to run the solution for (1-25)
    day: usize,

    /// The input data file. Will look for `data/day<num>.txt` by default
    input: Option<PathBuf>,
}

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn as_result<A: ToString, B: ToString>((a, b): (A, Option<B>)) -> (String, Option<String>) {
    (a.to_string(), b.map(|answer| answer.to_string()))
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let input = opts
        .input
        .unwrap_or_else(|| format!("data/day{}.txt", opts.day).into());

    #[allow(
        overlapping_range_endpoints,
        unreachable_patterns,
        clippy::match_overlapping_arm
    )]
    let (a, b): (String, Option<String>) = match opts.day {
        1 => as_result(day1::main(&input)?),
        2 => as_result(day2::main(&input)?),
        3 => as_result(day3::main(&input)?),
        4 => as_result(day4::main(&input)?),
        5 => as_result(day5::main(&input)?),
        6 => as_result(day6::main(&input)?),
        7 => as_result(day7::main(&input)?),
        1..=25 => return Err(anyhow!("No implementation for this day yet")),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    println!("A: {}", pad_newlines(a));
    if let Some(b) = b {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}
