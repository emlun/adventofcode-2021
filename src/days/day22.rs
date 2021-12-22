use crate::common::Solution;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Cuboid<T> = (RangeInclusive<T>, RangeInclusive<T>, RangeInclusive<T>);

pub fn solve(lines: &[String]) -> Solution {
    let steps: Vec<(bool, Cuboid<i64>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut splits = l.split(' ');
            let on = splits.next().unwrap() == "on";
            let mut coords = splits.next().unwrap().split(',').map(|coord| {
                let mut parts = coord.split('=').nth(1).unwrap().split("..");
                parts.next().unwrap().parse().unwrap()..=parts.next().unwrap().parse().unwrap()
            });
            (
                on,
                (
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                ),
            )
        })
        .collect();

    let cubes_on = steps
        .iter()
        .map(|(on, (xs, ys, zs))| {
            (
                on,
                (
                    std::cmp::max(*xs.start(), -50)..=std::cmp::min(50, *xs.end()),
                    std::cmp::max(*ys.start(), -50)..=std::cmp::min(50, *ys.end()),
                    std::cmp::max(*zs.start(), -50)..=std::cmp::min(50, *zs.end()),
                ),
            )
        })
        .filter(|(_, (xs, ys, zs))| !xs.is_empty() && !ys.is_empty() && !zs.is_empty())
        .fold(HashSet::new(), |mut cubes_on, (on, (xs, ys, zs))| {
            if *on {
                for x in xs {
                    for y in ys.clone() {
                        for z in zs.clone() {
                            cubes_on.insert((x, y, z));
                        }
                    }
                }
            } else {
                for x in xs {
                    for y in ys.clone() {
                        for z in zs.clone() {
                            cubes_on.remove(&(x, y, z));
                        }
                    }
                }
            }
            cubes_on
        });

    let sol_a = cubes_on.len();
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
