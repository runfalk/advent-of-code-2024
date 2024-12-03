use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

enum Instruction {
    Enable,
    Disable,
    Mul(usize, usize),
}

fn part_a(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .map(|inst| match inst {
            Instruction::Enable => 0,
            Instruction::Disable => 0,
            Instruction::Mul(a, b) => a * b,
        })
        .sum()
}

fn part_b(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .fold((true, 0), |(enabled, acc), inst| match inst {
            Instruction::Enable => (true, acc),
            Instruction::Disable => (false, acc),
            Instruction::Mul(a, b) => (enabled, acc + if enabled { a * b } else { 0 }),
        })
        .1
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let muls = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let data = std::fs::read_to_string(path).context("Failed to read file")?;
    let instructions = muls
        .captures_iter(&data)
        .map(|inst| {
            Ok(match &inst[0] {
                "do()" => Instruction::Enable,
                "don't()" => Instruction::Disable,
                _ => Instruction::Mul(inst[1].parse()?, inst[2].parse()?),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((part_a(&instructions), Some(part_b(&instructions))))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(3, 173517243, 100450138);
}
