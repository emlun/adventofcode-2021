use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve_sub<'a, I: Iterator<Item = &'a ((i64, i64), (i64, i64))>>(segments: I) -> usize {
    let mut point_counts: HashMap<(i64, i64), usize> = HashMap::new();
    for ((x1, y1), (x2, y2)) in segments {
        if x1 == x2 {
            for (x, y) in (std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2)).map(|y| (x1, y)) {
                *point_counts.entry((*x, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for (x, y) in (std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2)).map(|x| (x, y1)) {
                *point_counts.entry((x, *y)).or_insert(0) += 1;
            }
        } else {
            let xs = std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2);
            let ys = std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2);
            if x1 <= x2 {
                if y1 <= y2 {
                    for (x, y) in xs.zip(ys) {
                        *point_counts.entry((x, y)).or_insert(0) += 1;
                    }
                } else {
                    for (x, y) in xs.zip(ys.rev()) {
                        *point_counts.entry((x, y)).or_insert(0) += 1;
                    }
                }
            } else {
                if y1 <= y2 {
                    for (x, y) in xs.rev().zip(ys) {
                        *point_counts.entry((x, y)).or_insert(0) += 1;
                    }
                } else {
                    for (x, y) in xs.rev().zip(ys.rev()) {
                        *point_counts.entry((x, y)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    point_counts.values().filter(|v| **v >= 2).count()
}

pub fn solve(lines: &[String]) -> Solution {
    let segments: Vec<((i64, i64), (i64, i64))> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut splits = l
                .split(" -> ")
                .flat_map(|part| part.split(","))
                .map(|s| s.parse().unwrap());
            (
                (splits.next().unwrap(), splits.next().unwrap()),
                (splits.next().unwrap(), splits.next().unwrap()),
            )
        })
        .collect();

    (
        solve_sub(
            segments
                .iter()
                .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2),
        )
        .to_string(),
        solve_sub(segments.iter()).to_string(),
    )
}
