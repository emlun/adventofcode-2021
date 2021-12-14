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
            if let Some((p1, p2)) = rules.get(pair) {
                *result.entry(p1).or_insert(0) += count;
                *result.entry(p2).or_insert(0) += count;
            }
            result
        },
    )
}

fn count_solution(polymer: &HashMap<&str, usize>, template: &str) -> usize {
    let mut elem_counts: HashMap<char, usize> =
        polymer
            .iter()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                *counts.entry(pair.chars().next().unwrap()).or_insert(0) += count;
                counts
            });
    *elem_counts
        .get_mut(&template.chars().last().unwrap())
        .unwrap() += 1;
    elem_counts.values().max().unwrap() - elem_counts.values().min().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let template = &lines[0];
    let rules: HashMap<&str, (String, String)> = lines[2..]
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split(" -> ");
            let i = halves.next().unwrap();
            let o = halves.next().unwrap();
            (
                i,
                (format!("{}{}", &i[0..1], o), format!("{}{}", o, &i[1..2])),
            )
        })
        .collect();
    let rules: HashMap<&str, (&str, &str)> = rules
        .iter()
        .map(|(k, (v1, v2))| (*k, (v1.as_str(), v2.as_str())))
        .collect();

    let pair_counts = (0..(template.len() - 1))
        .map(|i| &template[i..=(i + 1)])
        .counts();
    let grown = (0..10).fold(pair_counts, |pair_counts, _| grow(&rules, pair_counts));
    let sol_a = count_solution(&grown, template);

    let grown = (10..40).fold(grown, |pair_counts, _| grow(&rules, pair_counts));
    let sol_b = count_solution(&grown, template);

    (sol_a.to_string(), sol_b.to_string())
}
