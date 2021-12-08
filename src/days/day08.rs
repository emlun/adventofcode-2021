use crate::common::Solution;
use std::collections::HashSet;

pub fn solve(lines: &[String]) -> Solution {
    let entries: Vec<(Vec<HashSet<u8>>, Vec<HashSet<u8>>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split('|').map(|s| s.trim());
            (
                halves
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.as_bytes().into_iter().map(|c| c - 0x61).collect())
                    .collect(),
                halves
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.as_bytes().into_iter().map(|c| c - 0x61).collect())
                    .collect(),
            )
        })
        .collect();

    let sol_a = entries
        .iter()
        .flat_map(|(_, o)| o)
        .filter(|o| o.len() == 2 || o.len() == 4 || o.len() == 3 || o.len() == 7)
        .count();

    (sol_a.to_string(), 0.to_string())
}
