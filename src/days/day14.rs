use crate::common::Solution;
use crate::util::iter::Countable;
use std::collections::HashMap;

fn grow<'a>(
    rules: &HashMap<&'a str, (&'a str, &'a str)>,
    polymer: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    polymer.into_iter().fold(
        HashMap::new(),
        |mut result, (pair, count): (&str, usize)| {
            if rules.contains_key(&pair) {
                if let Some((p1, p2)) = rules.get(pair) {
                    *result.entry(p1).or_insert(0) += count;
                    *result.entry(p2).or_insert(0) += count;
                }
                result
            } else {
                result
            }
        },
    )
}

fn pairs<'a, 'b>(s: &'a str) -> Vec<&'a str> {
    (0..(s.len() - 1)).map(|i| &s[i..=(i + 1)]).collect()
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

    let rules2: HashMap<&str, (String, String)> = rules
        .iter()
        .map(|(k, v)| {
            (
                *k,
                (format!("{}{}", &k[0..1], v), format!("{}{}", v, &k[1..2])),
            )
        })
        .collect();
    let rules22: HashMap<&str, (&str, &str)> = rules2
        .iter()
        .map(|(k, (v1, v2))| (*k, (v1.as_str(), v2.as_str())))
        .collect();

    let pair_counts = pairs(template).into_iter().counts();
    let grown = (0..10).fold(pair_counts, |pair_counts, _| grow(&rules22, pair_counts));

    let mut elem_counts: HashMap<char, usize> =
        grown
            .iter()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                let mut ch = pair.chars();
                *counts.entry(ch.next().unwrap()).or_insert(0) += count;
                counts
            });
    *elem_counts
        .get_mut(&template.chars().last().unwrap())
        .unwrap() += 1;

    let sol_a = elem_counts.values().max().unwrap() - elem_counts.values().min().unwrap();

    let grown = (10..40).fold(grown, |pair_counts, _| grow(&rules22, pair_counts));
    let mut elem_counts: HashMap<char, usize> =
        grown
            .iter()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                let mut ch = pair.chars();
                *counts.entry(ch.next().unwrap()).or_insert(0) += count;
                counts
            });
    *elem_counts
        .get_mut(&template.chars().last().unwrap())
        .unwrap() += 1;

    let sol_b = elem_counts.values().max().unwrap() - elem_counts.values().min().unwrap();

    (sol_a.to_string(), sol_b.to_string())
}
