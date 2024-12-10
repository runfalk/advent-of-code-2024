use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

fn score_trailheads(
    height_map: &HashMap<u32, HashSet<(isize, isize)>>,
    overlapping_paths: bool,
) -> usize {
    let mut trailhead_score = 0;
    for start in height_map.get(&0).iter().flat_map(|p| p.iter().copied()) {
        let mut to_visit = vec![(0u32, start)];

        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((elevation, (x, y))) = to_visit.pop() {
            if elevation == 9 {
                trailhead_score += 1;
                continue;
            }

            let next_elevation = elevation + 1;
            let Some(neighbors) = height_map.get(&next_elevation) else {
                continue;
            };
            for n in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                if neighbors.contains(&n) && (overlapping_paths || visited.insert(n)) {
                    to_visit.push((next_elevation, n));
                }
            }
        }
    }
    trailhead_score
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut height_map: HashMap<u32, HashSet<(isize, isize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c
                .to_digit(10)
                .ok_or_else(|| anyhow!("Invalid height map {}, at {}x{}", c, x, y))?;
            height_map
                .entry(height)
                .or_default()
                .insert((x as isize, y as isize));
        }
    }

    Ok((
        score_trailheads(&height_map, false),
        Some(score_trailheads(&height_map, true)),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(10, 746, 1541);
}
