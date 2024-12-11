use std::collections::HashMap;

use anyhow::Result;

fn blink(stones: &[usize], num_blinks: usize) -> usize {
    let mut a: HashMap<usize, usize> = HashMap::new();
    for stone in stones.iter().copied() {
        *a.entry(stone).or_default() += 1
    }

    for _ in 0..num_blinks {
        let mut b: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in a.into_iter() {
            if stone == 0 {
                *b.entry(stone + 1).or_default() += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let stone_str = stone.to_string();
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                *b.entry(left.parse().unwrap()).or_default() += count;
                *b.entry(right.parse().unwrap()).or_default() += count;
            } else {
                *b.entry(stone * 2024).or_default() += count;
            }
        }
        a = b;
    }
    a.into_values().sum()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let stones = input
        .split_whitespace()
        .map(|stone_str| stone_str.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    Ok((blink(&stones, 25), Some(blink(&stones, 75))))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(11, 216_996, 257_335_372_288_947);
}
