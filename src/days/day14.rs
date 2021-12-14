use crate::common::Solution;
use crate::util::iter::Countable;
use std::collections::HashMap;
use std::collections::HashSet;

fn grow(rules: &[(usize, usize)], polymer: Vec<usize>) -> Vec<usize> {
    let l = polymer.len();
    polymer
        .into_iter()
        .enumerate()
        .fold(vec![0; l], |mut result, (pair, count): (usize, usize)| {
            let (p1, p2) = rules[pair];
            result[p1] += count;
            result[p2] += count;
            result
        })
}

fn count_solution(int_to_name: &[&str], polymer: &[usize], template: &str) -> usize {
    let mut elem_counts: HashMap<char, usize> =
        polymer
            .iter()
            .enumerate()
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

    let int_to_name: Vec<&str> = rules
        .keys()
        .copied()
        .chain(rules.values().flat_map(|(a, b)| {
            Some(a.as_str())
                .into_iter()
                .chain(Some(b.as_str()).into_iter())
        }))
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let name_to_int: HashMap<&str, usize> = int_to_name
        .iter()
        .enumerate()
        .map(|(k, v)| (*v, k))
        .collect();
    let rules_int: Vec<(usize, usize)> = (0..int_to_name.len())
        .map(|i| {
            let (a, b) = &rules[int_to_name[i]];
            (name_to_int[&a.as_str()], name_to_int[&b.as_str()])
        })
        .collect();

    let polymer: Vec<usize> = (0..(template.len() - 1))
        .map(|i| name_to_int[&&template[i..=(i + 1)]])
        .counts()
        .into_iter()
        .fold(vec![0; int_to_name.len()], |mut counts, (i, count)| {
            counts[i] = count;
            counts
        });
    let grown = (0..10).fold(polymer, |polymer, _| grow(&rules_int, polymer));
    let sol_a = count_solution(&int_to_name, &grown, template);

    let grown = (10..40).fold(grown, |polymer, _| grow(&rules_int, polymer));
    let sol_b = count_solution(&int_to_name, &grown, template);

    (sol_a.to_string(), sol_b.to_string())
}
