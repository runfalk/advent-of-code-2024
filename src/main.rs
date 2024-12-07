#![deny(clippy::dbg_macro)]

use anyhow::{anyhow, Context as _, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

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

fn run<F: FnOnce(&str) -> Result<(A, Option<B>)>, A: ToString, B: ToString>(
    f: F,
    input: &str,
) -> Result<()> {
    let start = Instant::now();
    let (a, b) = f(input)?;
    let time = Instant::now().saturating_duration_since(start);

    println!("A: {}", pad_newlines(a.to_string()));
    if let Some(b) = b {
        println!("B: {}", pad_newlines(b.to_string()));
    }
    println!();
    println!("Time: {:.3} seconds", time.as_secs_f64());

    Ok(())
}

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn main() -> Result<()> {
    let opts = Options::parse();

    #[allow(
        overlapping_range_endpoints,
        unreachable_patterns,
        clippy::match_overlapping_arm
    )]
    let solution = match opts.day {
        1 => day1::main,
        2 => day2::main,
        3 => day3::main,
        4 => day4::main,
        5 => day5::main,
        6 => day6::main,
        7 => day7::main,
        day @ 1..=25 => return Err(anyhow!("No implementation for day {} yet", day)),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    let input_path = opts
        .input
        .unwrap_or_else(|| format!("data/day{}.txt", opts.day).into());
    let input = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to open input file {:?}", input_path))?;
    run(solution, &input)
}
