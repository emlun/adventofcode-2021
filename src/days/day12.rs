use crate::common::Solution;
use std::collections::HashMap;

fn count_paths<'a, 'b>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    current: &'b &'a str,
    small2_spent: bool,
    smalls: Vec<&'b &'a str>,
) -> usize {
    map[current]
        .iter()
        .map(|next| {
            if next == &"end" {
                1
            } else if next != &"start" {
                let is_small = next.chars().next().unwrap().is_lowercase();
                let small_visited = is_small && smalls.contains(&next);
                if !small2_spent || !small_visited {
                    count_paths(map, next, small2_spent || small_visited, {
                        let mut smalls = smalls.clone();
                        if is_small {
                            smalls.push(next);
                        }
                        smalls
                    })
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: HashMap<&str, Vec<&str>> =
        lines
            .iter()
            .filter(|l| !l.is_empty())
            .fold(HashMap::new(), |mut map, l| {
                let mut halves = l.split('-');
                let a: &str = halves.next().unwrap();
                let b: &str = halves.next().unwrap();
                map.entry(a).or_insert_with(Vec::new).push(b);
                map.entry(b).or_insert_with(Vec::new).push(a);
                map
            });

    let sol_a = count_paths(&map, &"start", true, Vec::new());
    let sol_b = count_paths(&map, &"start", false, Vec::new());

    (sol_a.to_string(), sol_b.to_string())
}
