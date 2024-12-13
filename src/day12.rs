use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut farm = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            farm.insert((x as isize, y as isize), c);
        }
    }

    let mut cost_a = 0;
    let mut cost_b = 0;
    let mut unchecked_gardens: BTreeSet<(isize, isize)> = farm.keys().copied().collect();
    while let Some((x, y)) = unchecked_gardens.pop_last() {
        let region = farm.get(&(x, y)).copied().unwrap();

        let mut gardens_in_region = HashSet::new();
        let mut to_visit = vec![(x, y)];
        while let Some((x, y)) = to_visit.pop() {
            gardens_in_region.insert((x, y));
            for neighbor in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                let Some(neighbor_region) = farm.get(&neighbor).copied() else {
                    continue;
                };
                if neighbor_region != region {
                    continue;
                }

                if unchecked_gardens.remove(&neighbor) {
                    to_visit.push(neighbor);
                }
            }
        }

        let area = gardens_in_region.len();
        let fences = gardens_in_region
            .iter()
            .copied()
            .flat_map(|(x, y)| [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)])
            .filter(|p| !gardens_in_region.contains(p))
            .count();

        let (min_x, max_x) = gardens_in_region
            .iter()
            .copied()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = gardens_in_region
            .iter()
            .copied()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();

        let mut sides = 0;
        for (dx, dy) in [(1, 0), (-1, 0)] {
            for x in min_x..=max_x {
                let mut is_following_side = false;
                for y in min_y..=max_y {
                    let has_neighbor = gardens_in_region.contains(&(x + dx, y + dy));
                    let is_side = gardens_in_region.contains(&(x, y)) && !has_neighbor;
                    if !is_following_side && is_side {
                        sides += 1;
                    }
                    is_following_side = is_side;
                }
            }
        }
        for (dx, dy) in [(0, -1), (0, 1)] {
            for y in min_y..=max_y {
                let mut is_following_side = false;
                for x in min_x..=max_x {
                    let has_neighbor = gardens_in_region.contains(&(x + dx, y + dy));
                    let is_side = gardens_in_region.contains(&(x, y)) && !has_neighbor;
                    if !is_following_side && is_side {
                        sides += 1;
                    }
                    is_following_side = is_side;
                }
            }
        }

        cost_a += fences * area;
        cost_b += sides * area;
    }

    Ok((cost_a, Some(cost_b)))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(12, 1_370_258, 805_814);

    const EXAMPLE_A: &str = dedent::dedent!(
        r#"
        AAAA
        BBCD
        BBCC
        EEEC
        "#
    );

    const EXAMPLE_B: &str = dedent::dedent!(
        r#"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        "#
    );

    const EXAMPLE_C: &str = dedent::dedent!(
        r#"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        "#
    );

    #[test]
    fn test_minimal() {
        assert_eq!(main("AAB").unwrap(), (16, Some(12)));
    }

    #[test]
    fn test_example_a() {
        assert_eq!(main(EXAMPLE_A).unwrap(), (140, Some(80)));
    }

    #[test]
    fn test_example_b() {
        assert_eq!(main(EXAMPLE_B).unwrap().0, 772);
    }

    #[test]
    fn test_example_c() {
        assert_eq!(main(EXAMPLE_C).unwrap(), (1930, Some(1206)));
    }
}
