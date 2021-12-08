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

    fn add_points<P: Into<CompactPoint>, I: Iterator<Item = P>>(
        counts: &mut HashMap<CompactPoint, usize>,
        points: I,
    ) {
        for point in points {
            *counts.entry(point.into()).or_insert(0) += 1;
        }
    }

    for ((x1, y1), (x2, y2)) in segments {
        if x1 == x2 {
            add_points(
                &mut straight_point_counts,
                (std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2)).map(|y| (*x1, y)),
            );
        } else if y1 == y2 {
            add_points(
                &mut straight_point_counts,
                (std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2)).map(|x| (x, *y1)),
            );
        } else {
            let xs = std::cmp::min(*x1, *x2)..=std::cmp::max(*x1, *x2);
            let ys = std::cmp::min(*y1, *y2)..=std::cmp::max(*y1, *y2);

            if x1 <= x2 {
                if y1 <= y2 {
                    add_points(&mut diag_point_counts, xs.zip(ys));
                } else {
                    add_points(&mut diag_point_counts, xs.zip(ys.rev()));
                }
            } else {
                if y1 <= y2 {
                    add_points(&mut diag_point_counts, xs.rev().zip(ys));
                } else {
                    add_points(&mut diag_point_counts, xs.rev().zip(ys.rev()));
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
