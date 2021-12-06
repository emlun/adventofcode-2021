use crate::common::Solution;

fn solve_a(fishes: Vec<usize>) -> usize {
    const DAYS: usize = 80;

    let fishes_after: Vec<usize> = (0..DAYS).fold(fishes, |mut fishes, _| {
        let zeros = fishes.remove(0);
        fishes[6] += zeros;
        fishes.push(zeros);
        fishes
    });

    fishes_after.into_iter().sum()
}

fn solve_b(lines: &[String]) -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let fishes: Vec<usize> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(','))
        .map(|s| s.parse().unwrap())
        .fold(vec![0; 9], |mut fishes, timer: usize| {
            fishes[timer] += 1;
            fishes
        });

    (solve_a(fishes).to_string(), solve_b(lines).to_string())
}
