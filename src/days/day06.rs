use crate::common::Solution;

fn simulate(fishes: Vec<usize>, days: usize) -> usize {
    let fishes_after: Vec<usize> = (0..days).fold(fishes, |mut fishes, _| {
        let zeros = fishes.remove(0);
        fishes[6] += zeros;
        fishes.push(zeros);
        fishes
    });

    fishes_after.into_iter().sum()
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

    (
        simulate(fishes.clone(), 80).to_string(),
        simulate(fishes, 256).to_string(),
    )
}
