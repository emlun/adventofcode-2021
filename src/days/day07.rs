use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let crabs: Vec<i64> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(','))
        .map(|s| s.parse().unwrap())
        .collect();

    let minx: i64 = *crabs.iter().min().unwrap();
    let maxx: i64 = *crabs.iter().max().unwrap();

    let sol_a: i64 = (minx..=maxx)
        .map(|x0| crabs.iter().map(|x| (*x - x0).abs()).sum())
        .min()
        .unwrap();

    let sol_b: i64 = (minx..=maxx)
        .map(|x0| {
            crabs
                .iter()
                .map(|x| {
                    let n = (*x - x0).abs();
                    n * (n + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    (sol_a.to_string(), sol_b.to_string())
}
