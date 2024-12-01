use anyhow::{anyhow, Context as _, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

fn part_a(mut first: Vec<usize>, mut second: Vec<usize>) -> usize {
    first.sort();
    second.sort();
    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part_b(first: Vec<usize>, second: Vec<usize>) -> usize {
    let mut lookup: HashMap<usize, usize> = HashMap::new();
    for v in second {
        *lookup.entry(v).or_insert(0) += 1;
    }
    first
        .into_iter()
        .map(|v| v * lookup.get(&v).copied().unwrap_or(0))
        .sum()
}

fn parse_line(line: Result<String, io::Error>) -> Result<(usize, usize)> {
    let line = line?;
    let mut pair = line.split_whitespace();
    let a = pair.next().ok_or_else(|| anyhow!("No list found"))?;
    let b = pair.next().ok_or_else(|| anyhow!("No second list found"))?;
    if let Some(c) = pair.next() {
        return Err(anyhow!("Found more than two lists ({:?})", c));
    }

    Ok((a.parse()?, b.parse()?))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut first = Vec::new();
    let mut second = Vec::new();

    let file = File::open(path)?;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let (a, b) = parse_line(line).with_context(|| format!("Failed to parse line {}", i + 1))?;
        first.push(a);
        second.push(b);
    }

    Ok((
        part_a(first.clone(), second.clone()),
        Some(part_b(first, second)),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(1, 1341714, 27384707);

    const FIRST_LIST: &[usize] = &[3, 4, 2, 1, 3, 3];

    const SECOND_LIST: &[usize] = &[4, 3, 5, 3, 9, 3];

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(FIRST_LIST.to_vec(), SECOND_LIST.to_vec()), 11);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(FIRST_LIST.to_vec(), SECOND_LIST.to_vec()), 31);
    }
}
