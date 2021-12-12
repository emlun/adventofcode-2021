use crate::common::Solution;
use std::collections::HashMap;

fn count_paths(
    map: &[usize],
    smalls: usize,
    start_int: usize,
    end_int: usize,
    current_int: usize,
    small2_spent: bool,
    smalls_visited: usize,
) -> usize {
    (0..(usize::BITS - map[current_int].leading_zeros()))
        .filter(|i| map[current_int] & (1 << i) != 0)
        .map(|i| i as usize)
        .map(|next_int: usize| {
            let next_bit = 1 << next_int;
            if next_int == end_int {
                1
            } else if next_int != start_int {
                let is_small = (smalls & next_bit) != 0;
                let small_visited = is_small && ((smalls_visited & next_bit) != 0);
                if !small2_spent || !small_visited {
                    count_paths(
                        map,
                        smalls,
                        start_int,
                        end_int,
                        next_int,
                        small2_spent || small_visited,
                        {
                            if is_small {
                                smalls_visited | next_bit
                            } else {
                                smalls_visited
                            }
                        },
                    )
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

    let name_ints: HashMap<&str, usize> =
        map.keys().enumerate().map(|(i, name)| (*name, i)).collect();

    let name_bits: HashMap<&str, usize> = map
        .keys()
        .enumerate()
        .map(|(i, name)| (*name, 1 << i))
        .collect();

    let small: usize = map
        .keys()
        .filter(|name| name.chars().next().unwrap().is_lowercase())
        .map(|name| name_bits[name])
        .sum();

    let l = map.len();
    let int_map: Vec<usize> =
        map.into_iter()
            .fold(vec![0; l], |mut result, (name, connections)| {
                result[name_ints[name]] = connections.into_iter().map(|conn| name_bits[conn]).sum();
                result
            });

    let start_int = name_ints["start"];
    let end_int = name_ints["end"];

    let sol_a = count_paths(&int_map, small, start_int, end_int, start_int, true, 0);
    let sol_b = count_paths(&int_map, small, start_int, end_int, start_int, false, 0);

    (sol_a.to_string(), sol_b.to_string())
}
