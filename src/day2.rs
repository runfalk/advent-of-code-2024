use anyhow::{Context as _, Result};
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

use itertools::Itertools;

fn is_report_safe(report: impl AsRef<[usize]>, skip: Option<usize>) -> bool {
    let mut was_ascending = None;
    for (prev, curr) in report
        .as_ref()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, v)| (Some(i) != skip).then_some(v))
        .tuple_windows()
    {
        let is_ascending = prev < curr;
        if was_ascending.unwrap_or(is_ascending) != is_ascending
            || !(1..=3).contains(&prev.abs_diff(curr))
        {
            return false;
        }
        was_ascending = Some(is_ascending);
    }
    true
}

fn parse_report(line: Result<String, io::Error>) -> Result<Vec<usize>> {
    let line = line?;
    line.split_whitespace().map(|v| Ok(v.parse()?)).collect()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let mut part_a = 0;
    let mut part_b = 0;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let report =
            parse_report(line).with_context(|| format!("Failed to parse line {}", i + 1))?;
        if is_report_safe(&report, None) {
            // Report is valid for both part a and b
            part_a += 1;
        } else if (0..report.len()).any(|i| is_report_safe(&report, Some(i))) {
            // Report is only valid for part b
            part_b += 1;
        }
    }

    // Add reports that are valid to part b, as we only count the ones that are valid only with the
    // problem dampener
    part_b += part_a;

    Ok((part_a, Some(part_b)))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(2, 639, 674);

    #[test]
    fn test_without_dampener() {
        assert!(is_report_safe(&[7, 6, 4, 2, 1], None));
        assert!(!is_report_safe(&[1, 2, 7, 8, 9], None));
        assert!(!is_report_safe(&[9, 7, 6, 2, 1], None));
        assert!(!is_report_safe(&[1, 3, 2, 4, 5], None));
        assert!(!is_report_safe(&[8, 6, 4, 4, 1], None));
        assert!(is_report_safe(&[1, 3, 6, 7, 9], None));
    }

    #[test]
    fn test_with_dampener() {
        assert!(is_report_safe(&[1, 3, 2, 4, 5], Some(1)));
        assert!(is_report_safe(&[8, 6, 4, 4, 1], Some(2)));

        // Edge case where the first value needs to be skipped to make the report valid
        assert!(!is_report_safe(&[8, 4, 5, 6, 7], None));
        assert!(is_report_safe(&[8, 4, 5, 6, 7], Some(0)));
    }
}
