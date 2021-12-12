use crate::common::Solution;
use std::collections::HashMap;

#[derive(Clone)]
struct Path<'a, 'b> {
    current: &'a &'b str,
    len: usize,
    smalls: Vec<&'a &'b str>,
}

fn count_paths<'a, 'b>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    path: Path<'b, 'a>,
    small2_spent: bool,
) -> usize {
    map[path.current]
        .iter()
        .map(|next| {
            if next == &"end" {
                1
            } else if next != &"start" && (!small2_spent || !path.smalls.contains(&&next)) {
                let is_small = next.chars().next().unwrap().is_lowercase();
                let mut smalls = path.smalls.clone();
                if is_small {
                    smalls.push(next);
                }
                count_paths(
                    map,
                    Path {
                        current: next,
                        len: path.len + 1,
                        smalls,
                    },
                    small2_spent || (is_small && path.smalls.contains(&next)),
                )
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

    let start = Path {
        current: &"start",
        len: 0,
        smalls: Vec::new(),
    };
    let sol_a = count_paths(&map, start.clone(), true);
    let sol_b = count_paths(&map, start, false);

    (sol_a.to_string(), sol_b.to_string())
}
