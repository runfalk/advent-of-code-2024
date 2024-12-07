use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn advance(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut guard = None;
    let mut map = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((x as isize, y as isize));
                }
                '^' => guard = Some((x as isize, y as isize)),
                _ => {}
            }
            max_x = max_x.max(x as isize);
        }
        max_y = max_y.max(y as isize);
    }

    let guard = guard.ok_or_else(|| anyhow!("No guard start location found"))?;
    let x_bounds = 0..=max_x;
    let y_bounds = 0..=max_y;

    // Find the path of the guard
    let mut visited = HashSet::new();
    let mut visited_with_direction = HashSet::new();
    let (mut x, mut y) = guard;
    let mut dir = Direction::Up;
    while x_bounds.contains(&x) && y_bounds.contains(&y) {
        visited.insert((x, y));
        if !visited_with_direction.insert((dir, x, y)) {
            return Err(anyhow!("Loop found without altering the map"));
        }
        let (nx, ny) = dir.advance((x, y));
        if map.contains(&(nx, ny)) {
            dir = dir.turn_right();
            continue;
        }
        (x, y) = (nx, ny);
    }

    let visited_positions = visited.len();

    // Use the original path to determine where we should try to inject obstacles
    let mut num_possible_obstacle_positions = 0;
    let obstacles_to_try: HashSet<_> = visited_with_direction
        .into_iter()
        .map(|(dir, x, y)| dir.advance((x, y)))
        .filter(|(x, y)| x_bounds.contains(x) && y_bounds.contains(y))
        .collect();
    for (ox, oy) in obstacles_to_try {
        let mut dir = Direction::Up;
        let (mut x, mut y) = guard;

        let mut visited = HashSet::new();
        while x_bounds.contains(&x) && y_bounds.contains(&y) {
            if !visited.insert((dir, x, y)) {
                num_possible_obstacle_positions += 1;
                break;
            }
            let (nx, ny) = dir.advance((x, y));
            if (nx == ox && ny == oy) || map.contains(&(nx, ny)) {
                dir = dir.turn_right();
                continue;
            }
            (x, y) = (nx, ny);
        }
    }

    Ok((visited_positions, Some(num_possible_obstacle_positions)))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(6, 5086, 1770);

    const EXAMPLE: &str = dedent::dedent!(
        r#"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
        "#
    );

    #[test]
    fn test_example() {
        assert_eq!(main(EXAMPLE).unwrap(), (41, Some(6)));
    }
}
