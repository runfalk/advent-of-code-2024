use anyhow::{anyhow, Result};

use itertools::Itertools;

fn part_a(blocks: Vec<Option<usize>>) -> usize {
    let num_used_blocks = blocks.iter().filter(|block| block.is_some()).count();
    let mut rev_blocks = blocks.iter().copied().rev().flatten();
    let mut compacted = Vec::new();
    for block in blocks.iter().copied().take(num_used_blocks) {
        match block {
            Some(id) => compacted.push(id),
            None => {
                compacted.push(rev_blocks.next().unwrap());
            }
        }
    }

    compacted.iter().enumerate().map(|(i, a)| i * a).sum()
}

fn part_b(mut blocks: Vec<Option<usize>>) -> usize {
    let mut end = blocks.len();
    while end > 0 {
        // Find start
        let id = blocks[end - 1];
        let len = blocks[..end]
            .iter()
            .rev()
            .take_while(|block| block == &&id)
            .count();
        let start = end - len;
        let (a, b) = blocks.split_at_mut(start);
        end = start;

        // Find suitable place to insert
        for free_start in 0..=(a.len().saturating_sub(len)) {
            let target = &mut a[free_start..];
            if target.len() < len || target[..len].iter().any(Option::is_some) {
                continue;
            }
            target[..len].swap_with_slice(&mut b[..len]);
        }
    }
    blocks
        .iter()
        .enumerate()
        .map(|(i, a)| i * a.unwrap_or(0))
        .sum()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut blocks = Vec::new();
    for (id, (used, free)) in input
        .trim_end()
        .chars()
        .chain(std::iter::once('0'))
        .tuples()
        .enumerate()
    {
        for _ in 0..used
            .to_digit(10)
            .ok_or_else(|| anyhow!("Invalid used specifier {}", used))?
        {
            blocks.push(Some(id));
        }

        for _ in 0..free
            .to_digit(10)
            .ok_or_else(|| anyhow!("Invalid free specifier {}", free))?
        {
            blocks.push(None);
        }
    }

    Ok((part_a(blocks.clone()), Some(part_b(blocks))))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(9, 6_200_294_120_911, 6_227_018_762_750);

    #[test]
    fn test_example() {
        assert_eq!(main("2333133121414131402").unwrap(), (1928, Some(2858)));
    }

    #[test]
    fn test_edge_case() {
        // https://www.reddit.com/r/adventofcode/comments/1hamyyn/2024_day_9_part_2_python/
        assert_eq!(main("2333133121414131499").unwrap(), (3630, Some(6204)));
    }
}
