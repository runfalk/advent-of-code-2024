use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn is_valid(rules: &HashMap<usize, HashSet<usize>>, update: &[usize]) -> bool {
    for (i, v) in update.iter().enumerate() {
        if rules
            .get(v)
            .map(|a| update[..i].iter().any(|x| a.contains(x)))
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

fn part_a(rules: &HashMap<usize, HashSet<usize>>, updates: &[impl AsRef<[usize]>]) -> usize {
    let mut n = 0;
    for update in updates
        .iter()
        .map(AsRef::as_ref)
        .filter(|update| is_valid(rules, update))
    {
        n += update[update.len() / 2];
    }
    n
}

fn part_b(rules: &HashMap<usize, HashSet<usize>>, updates: &[impl AsRef<[usize]>]) -> usize {
    let mut n = 0;
    for update in updates.iter().map(AsRef::as_ref) {
        if is_valid(rules, update) {
            continue;
        }

        let mut reordered_update = update.to_vec();
        reordered_update.sort_by(|a, b| {
            if rules.get(a).map(|after| after.contains(b)).unwrap_or(false) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if is_valid(rules, &reordered_update) {
            n += reordered_update[reordered_update.len() / 2];
        }
    }
    n
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = fs::read_to_string(path)?;
    let (rules_str, update_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to split rules and updates"))?;

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    for rule_str in rules_str.lines() {
        let (before, after) = rule_str
            .split_once('|')
            .ok_or_else(|| anyhow!("No | found in rule"))?;
        rules
            .entry(before.parse()?)
            .or_default()
            .insert(after.parse()?);
    }

    let updates = update_str
        .lines()
        .map(|l| {
            l.split(',')
                .map(|i| Ok(i.parse()?))
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Vec<Vec<usize>>>>()?;

    Ok((part_a(&rules, &updates), Some(part_b(&rules, &updates))))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(5, 4774, 6004);
}
