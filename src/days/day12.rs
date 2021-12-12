use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Path<'a, 'b> {
    current: &'a &'b str,
    len: usize,
    smalls: HashSet<&'a &'b str>,
    small2_spent: bool,
}

fn count_paths<'a>(map: &HashMap<&'a str, HashSet<&'a str>>, small_twice: bool) -> usize {
    let mut count = 0;
    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path {
        current: &"start",
        len: 0,
        smalls: HashSet::new(),
        small2_spent: small_twice,
    });
    while let Some(path) = queue.pop_front() {
        if path.current == &"end" {
            count += 1;
        } else {
            queue.extend(
                map[path.current]
                    .iter()
                    .filter(|next| {
                        next != &&"start" && (!path.small2_spent || !path.smalls.contains(*next))
                    })
                    .map(|next| {
                        let is_small = next.chars().next().unwrap().is_lowercase();
                        Path {
                            current: next,
                            len: path.len + 1,
                            smalls: if is_small {
                                let mut s = path.smalls.clone();
                                s.insert(next);
                                s
                            } else {
                                path.smalls.clone()
                            },
                            small2_spent: path.small2_spent
                                || (is_small && path.smalls.contains(next)),
                        }
                    }),
            );
        }
    }
    count
}

pub fn solve(lines: &[String]) -> Solution {
    let map: HashMap<&str, HashSet<&str>> =
        lines
            .iter()
            .filter(|l| !l.is_empty())
            .fold(HashMap::new(), |mut map, l| {
                let mut halves = l.split('-');
                let a: &str = halves.next().unwrap();
                let b: &str = halves.next().unwrap();
                map.entry(a).or_insert_with(HashSet::new).insert(b);
                map.entry(b).or_insert_with(HashSet::new).insert(a);
                map
            });

    let sol_a = count_paths(&map, true);
    let sol_b = count_paths(&map, false);

    (sol_a.to_string(), sol_b.to_string())
}
