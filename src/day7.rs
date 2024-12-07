use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

fn parse_eq(l: io::Result<String>) -> Result<(usize, Vec<usize>)> {
    let l = l?;
    let Some((id_str, eq_str)) = l.split_once(": ") else {
        return Err(anyhow!("No separator between test value and numbers found"));
    };
    let nums = eq_str
        .split(' ')
        .map(|s| Ok(s.parse()?))
        .collect::<Result<Vec<_>>>()?;
    Ok((id_str.parse()?, nums))
}

fn is_valid_eq(test_value: usize, nums: &[usize], use_concat: bool) -> bool {
    let num_operators: usize = if use_concat { 3 } else { 2 };
    let num_combinations = num_operators.pow((nums.len() - 1) as u32);
    for mut ops in 0..num_combinations {
        let mut it = nums.iter().copied();
        let Some(mut acc) = it.next() else {
            return false;
        };
        for curr in it {
            match ops % num_operators {
                0 => acc += curr,
                1 => acc *= curr,
                2 => {
                    let num_digits = curr.checked_ilog10().unwrap_or(0) + 1;
                    acc = acc * 10usize.pow(num_digits) + curr;
                }
                _ => unreachable!(),
            }
            ops /= num_operators;
        }
        if acc == test_value {
            return true;
        }
    }
    false
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut a = 0;
    let mut b = 0;

    let file = File::open(path)?;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let (test_value, nums) =
            parse_eq(line).with_context(|| anyhow!("Failed to read line {}", i + 1))?;
        if is_valid_eq(test_value, &nums, false) {
            a += test_value;
        }
        if is_valid_eq(test_value, &nums, true) {
            b += test_value;
        }
    }
    Ok((a, Some(b)))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(7, 1_289_579_105_366, 92_148_721_834_692);
}
