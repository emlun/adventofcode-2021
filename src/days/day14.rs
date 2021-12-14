use crate::common::Solution;
use crate::util::iter::Countable;
use std::collections::HashMap;
use std::collections::HashSet;

fn grow(rules: &[(usize, usize)], polymer: HashMap<usize, usize>) -> HashMap<usize, usize> {
    polymer.into_iter().fold(
        HashMap::new(),
        |mut result, (pair, count): (usize, usize)| {
            let (p1, p2) = rules[pair];
            *result.entry(p1).or_insert(0) += count;
            *result.entry(p2).or_insert(0) += count;
            result
        },
    )
}

fn count_solution(
    int_to_name: &HashMap<usize, &&str>,
    polymer: &HashMap<usize, usize>,
    template: &str,
) -> usize {
    let mut elem_counts: HashMap<char, usize> =
        polymer
            .iter()
            .fold(HashMap::new(), |mut counts, (pair, count)| {
                *counts
                    .entry(int_to_name[pair].chars().next().unwrap())
                    .or_insert(0) += count;
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
    let pairs: HashSet<&&str> = rules
        .keys()
        .chain(
            rules
                .values()
                .flat_map(|(a, b)| Some(a).into_iter().chain(Some(b).into_iter())),
        )
        .collect();
    let int_to_name: HashMap<usize, &&str> = pairs.iter().copied().enumerate().collect();
    let name_to_int: HashMap<&&str, usize> =
        pairs.into_iter().enumerate().map(|(i, n)| (n, i)).collect();
    let rules_int: Vec<(usize, usize)> = (0..=*int_to_name.keys().max().unwrap())
        .map(|i| {
            let (a, b) = &rules[int_to_name[&i]];
            (name_to_int[a], name_to_int[b])
        })
        .collect();

    let pair_counts: HashMap<usize, usize> = (0..(template.len() - 1))
        .map(|i| name_to_int[&&template[i..=(i + 1)]])
        .counts();
    let grown = (0..10).fold(pair_counts, |pair_counts, _| grow(&rules_int, pair_counts));
    let sol_a = count_solution(&int_to_name, &grown, template);

    let grown = (10..40).fold(grown, |pair_counts, _| grow(&rules_int, pair_counts));
    let sol_b = count_solution(&int_to_name, &grown, template);

    (sol_a.to_string(), sol_b.to_string())
}
