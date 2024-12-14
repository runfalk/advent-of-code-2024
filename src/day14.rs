use std::collections::HashSet;

use anyhow::{anyhow, Result};
use regex::Regex;

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let re = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let mut robots: Vec<(isize, isize, isize, isize)> = Vec::new();
    for line in input.lines() {
        let Some((_, [x, y, dx, dy])) = re.captures(line).map(|c| c.extract()) else {
            return Err(anyhow!("Failed to parse line {line:?}"));
        };
        robots.push((
            x.parse().unwrap(),
            y.parse().unwrap(),
            dx.parse().unwrap(),
            dy.parse().unwrap(),
        ));
    }

    let width: isize = 101;
    let height: isize = 103;

    let mut part_a = None;
    let mut part_b = None;

    for i in 1.. {
        if part_a.is_some() && part_b.is_some() {
            break;
        }

        let mut points = HashSet::new();
        for (x, y, dx, dy) in robots.iter_mut() {
            *x = (*x + *dx).rem_euclid(width);
            *y = (*y + *dy).rem_euclid(height);

            points.insert((*x, *y));
        }

        if i == 100 {
            let mut quadrants = [0usize; 4];
            let h = width / 2;
            let v = height / 2;
            for (x, y, _, _) in robots.iter().copied() {
                quadrants[0] += usize::from(x < h && y < v);
                quadrants[1] += usize::from(x > h && y < v);
                quadrants[2] += usize::from(x < h && y > v);
                quadrants[3] += usize::from(x > h && y > v);
            }
            part_a = Some(quadrants.into_iter().product());
        }

        // The trunk has a 3x3 and it seems like this will only trigger for the easter egg frame
        for (x, y) in points.iter().copied() {
            let neighbors = [
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ];
            let has_christmas_tree = neighbors.into_iter().all(|p| points.contains(&p));
            if has_christmas_tree {
                // Only register the first occurrence
                part_b = part_b.or(Some(i));
            }
        }
    }

    Ok((
        part_a.ok_or_else(|| anyhow!("No solution for part A"))?,
        Some(part_b.ok_or_else(|| anyhow!("No solution for part B"))?),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(14, 231_782_040, 6475);
}
