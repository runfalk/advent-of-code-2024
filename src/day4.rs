use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct CharLookup(HashMap<char, HashSet<(isize, isize)>>);

impl CharLookup {
    /// Iterate over all positions that contain the given character
    fn positions(&self, c: char) -> impl Iterator<Item = &(isize, isize)> {
        self.0.get(&c).into_iter().flat_map(|p| p.iter())
    }

    /// Return true if the given character is shown at the given position
    fn has(&self, c: char, p: &(isize, isize)) -> bool {
        self.0.get(&c).map(|x| x.contains(p)).unwrap_or(false)
    }
}

fn part_a(map: &CharLookup) -> usize {
    let mut n = 0;
    for (x, y) in map.positions('X') {
        n += [-1isize, 0, 1]
            .into_iter()
            .cartesian_product([-1isize, 0, 1])
            .filter(|(x_step, y_step)| {
                ['X', 'M', 'A', 'S']
                    .into_iter()
                    .enumerate()
                    .all(|(step, c)| {
                        let step = step as isize;
                        let p = (x + x_step * step, y + y_step * step);
                        map.has(c, &p)
                    })
            })
            .count();
    }
    n
}

fn part_b(map: &CharLookup) -> usize {
    let mut n = 0;
    for (x, y) in map.positions('A') {
        let matches = [-1isize, 1]
            .into_iter()
            .cartesian_product([-1isize, 1])
            .filter(|(x_step, y_step)| {
                let start = (x - x_step, y - y_step);
                let end = (x + x_step, y + y_step);
                map.has('M', &start) && map.has('S', &end)
            })
            .count();
        if matches == 2 {
            n += 1;
        }
    }
    n
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut map: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            map.entry(c).or_default().insert((x as isize, y as isize));
        }
    }

    let lookup = CharLookup(map);

    Ok((part_a(&lookup), Some(part_b(&lookup))))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(4, 2297, 1745);

    const EXAMPLE: &str = dedent::dedent!(
        r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#
    );

    #[test]
    fn test_example() {
        assert_eq!(main(EXAMPLE).unwrap(), (18, Some(9)));
    }
}
