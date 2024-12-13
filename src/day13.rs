use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use regex::Regex;

use itertools::Itertools;

fn parse_line(s: &str) -> Result<(usize, usize)> {
    static CACHED_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = CACHED_REGEX.get_or_init(|| {
        Regex::new(r"^(?:Button A|Button B|Prize): X[+=](\d+), Y[+=](\d+)$").unwrap()
    });
    let Some((_, [x, y])) = re.captures(s).map(|caps| caps.extract()) else {
        return Err(anyhow!("{s:?} is not a valid input line"));
    };

    // We unwrap here because the regex only matches valid integers
    Ok((x.parse().unwrap(), y.parse().unwrap()))
}

fn cost_for_prize(
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    prize_x: usize,
    prize_y: usize,
) -> Option<usize> {
    // Use Cramer's rule
    // https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems
    let a = (prize_x * by).abs_diff(bx * prize_y) / (ax * by).abs_diff(ay * bx);
    let b = (ax * prize_y).abs_diff(prize_x * ay) / (ax * by).abs_diff(bx * ay);
    if ax * a + bx * b == prize_x && ay * a + by * b == prize_y {
        Some(3 * a + b)
    } else {
        None
    }
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let b_offset = 10_000_000_000_000usize;

    let mut part_a = 0;
    let mut part_b = 0;
    let mut lines = input.lines();
    while let Some((button_a_str, button_b_str, prize_str)) = lines.next_tuple() {
        // Skip blank line if there is one
        lines.next();

        let (button_a_x, button_a_y) = parse_line(button_a_str)?;
        let (button_b_x, button_b_y) = parse_line(button_b_str)?;
        let (prize_x, prize_y) = parse_line(prize_str)?;

        part_a += cost_for_prize(
            button_a_x, button_a_y, button_b_x, button_b_y, prize_x, prize_y,
        )
        .unwrap_or(0);
        part_b += cost_for_prize(
            button_a_x,
            button_a_y,
            button_b_x,
            button_b_y,
            prize_x + b_offset,
            prize_y + b_offset,
        )
        .unwrap_or(0);
    }
    Ok((part_a, Some(part_b)))
}

#[cfg(test)]
mod test {
    use super::*;
    test_real_input!(13, 37_297, 83_197_086_729_371);
}
