use crate::common::Solution;
use crate::util::iter::Countable;
use std::collections::HashMap;
use std::collections::HashSet;

fn grow(rules: &HashMap<&str, &str>, polymer: String) -> String {
    (0..(polymer.len() - 1))
        .map(|i| {
            format!(
                "{}{}",
                &polymer[i..=i],
                rules.get(&polymer[i..=(i + 1)]).unwrap_or(&""),
            )
        })
        .collect::<String>()
        + &polymer[(polymer.len() - 1)..=(polymer.len() - 1)]
}

pub fn solve(lines: &[String]) -> Solution {
    let template = &lines[0];
    let rules: HashMap<&str, &str> = lines[2..]
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split(" -> ");
            (halves.next().unwrap(), halves.next().unwrap())
        })
        .collect();

    let grown = (0..10).fold(template.to_string(), |polymer, _| grow(&rules, polymer));
    let counts = grown.chars().counts();

    let sol_a = counts.values().max().unwrap() - counts.values().min().unwrap();
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
