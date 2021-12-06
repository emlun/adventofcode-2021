use crate::common::Solution;

fn simulate(fishes: Vec<usize>, start_day: usize, days: usize) -> Vec<usize> {
    (start_day..days).fold(fishes, |mut fishes, day| {
        let i0 = day % 9;
        fishes[(i0 + 7) % 9] += fishes[i0];
        fishes
    })
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

    let fishes_a = simulate(fishes, 0, 80);
    let solution_a: usize = fishes_a.iter().sum();
    let fishes_b = simulate(fishes_a, 80, 256);
    let solution_b: usize = fishes_b.into_iter().sum();

    (solution_a.to_string(), solution_b.to_string())
}
