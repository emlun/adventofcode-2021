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
    let sol = (minx..=maxx)
        .map(|x0| crabs.iter().map(|x| (*x - x0).abs()).sum::<i64>())
        .min()
        .unwrap();

    (sol.to_string(), 0.to_string())
}
