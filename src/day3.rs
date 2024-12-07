use anyhow::Result;
use regex::Regex;

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

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let muls = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let instructions = muls
        .captures_iter(input)
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

    const EXAMPLE_A: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_example_a() {
        assert_eq!(main(EXAMPLE_A).unwrap().0, 161);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(main(EXAMPLE_B).unwrap().1.unwrap(), 48);
    }
}
