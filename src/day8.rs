use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use itertools::Itertools;

fn part_a(
    antennas_by_freq: &HashMap<char, HashSet<(isize, isize)>>,
    x_bounds: RangeInclusive<isize>,
    y_bounds: RangeInclusive<isize>,
) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in antennas_by_freq.values() {
        for ((ax, ay), (bx, by)) in antennas.iter().tuple_combinations() {
            let (dx, dy) = (ax - bx, ay - by);
            for (ax, ay) in [(ax + dx, ay + dy), (bx - dx, by - dy)] {
                if x_bounds.contains(&ax) && y_bounds.contains(&ay) {
                    antinodes.insert((ax, ay));
                }
            }
        }
    }
    antinodes.len()
}

fn part_b(
    antennas_by_freq: &HashMap<char, HashSet<(isize, isize)>>,
    x_bounds: RangeInclusive<isize>,
    y_bounds: RangeInclusive<isize>,
) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in antennas_by_freq.values() {
        for ((ax, ay), (bx, by)) in antennas.iter().tuple_combinations() {
            let (dx, dy) = (ax - bx, ay - by);

            for step in 0.. {
                let (ax, ay) = (ax + step * dx, ay + step * dy);
                if !x_bounds.contains(&ax) || !y_bounds.contains(&ay) {
                    break;
                }
                antinodes.insert((ax, ay));
            }

            for step in 0.. {
                let (ax, ay) = (bx - step * dx, by - step * dy);
                if !x_bounds.contains(&ax) || !y_bounds.contains(&ay) {
                    break;
                }
                antinodes.insert((ax, ay));
            }
        }
    }
    antinodes.len()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut antennas_by_freq = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        max_y = max_y.max(y);
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            max_x = max_x.max(x);
            if c == '.' || c == '#' {
                continue;
            }
            antennas_by_freq
                .entry(c)
                .or_insert_with(HashSet::new)
                .insert((x, y));
        }
    }

    let x_bounds = 0..=max_x;
    let y_bounds = 0..=max_y;

    Ok((
        part_a(&antennas_by_freq, x_bounds.clone(), y_bounds.clone()),
        Some(part_b(&antennas_by_freq, x_bounds, y_bounds)),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(8, 276, 991);

    const EXAMPLE: &str = dedent::dedent!(
        r#"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "#
    );

    #[test]
    fn test_example() {
        assert_eq!(main(EXAMPLE).unwrap(), (14, Some(34)));
    }
}
