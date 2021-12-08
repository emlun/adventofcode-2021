use crate::common::Solution;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct CompactPoint(i64);
impl From<(i32, i32)> for CompactPoint {
    fn from((x, y): (i32, i32)) -> Self {
        Self((i64::from(x) << 32) | i64::from(y))
    }
}

fn solve_sub<'a, I: Iterator<Item = &'a ((i32, i32), (i32, i32))>>(segments: I) -> (usize, usize) {
    let mut straight_point_counts: HashMap<CompactPoint, usize> = HashMap::new();
    let mut diag_point_counts: HashMap<CompactPoint, usize> = HashMap::new();

    for ((x1, y1), (x2, y2)) in segments {
        if x1 == x2 {
            for y in std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2) {
                *straight_point_counts.entry((*x1, y).into()).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2) {
                *straight_point_counts.entry((x, *y1).into()).or_insert(0) += 1;
            }
        } else {
            let xs = std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2);
            let ys = std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2);
            if x1 <= x2 {
                if y1 <= y2 {
                    for (x, y) in xs.zip(ys) {
                        *diag_point_counts.entry((x, y).into()).or_insert(0) += 1;
                    }
                } else {
                    for (x, y) in xs.zip(ys.rev()) {
                        *diag_point_counts.entry((x, y).into()).or_insert(0) += 1;
                    }
                }
            } else {
                if y1 <= y2 {
                    for (x, y) in xs.rev().zip(ys) {
                        *diag_point_counts.entry((x, y).into()).or_insert(0) += 1;
                    }
                } else {
                    for (x, y) in xs.rev().zip(ys.rev()) {
                        *diag_point_counts.entry((x, y).into()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    for (xy, c) in straight_point_counts.iter() {
        if *c >= 2 {
            *diag_point_counts.entry(*xy).or_insert(0) += c;
        } else {
            diag_point_counts.entry(*xy).and_modify(|cc| *cc += 1);
        }
    }
    (
        straight_point_counts.values().filter(|v| **v >= 2).count(),
        diag_point_counts.values().filter(|v| **v >= 2).count(),
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let segments: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut splits = l
                .split(" -> ")
                .flat_map(|part| part.split(','))
                .map(|s| s.parse().unwrap());
            (
                (splits.next().unwrap(), splits.next().unwrap()),
                (splits.next().unwrap(), splits.next().unwrap()),
            )
        })
        .collect();

    let (sol_a, sol_b) = solve_sub(segments.iter());
    (sol_a.to_string(), sol_b.to_string())
}
