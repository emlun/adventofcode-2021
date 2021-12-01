use crate::common::Solution;

fn solve_a(numbers: &[i32]) -> usize {
    (1..numbers.len())
        .filter(|i| numbers[i - 1] < numbers[*i])
        .count()
}

fn solve_b(numbers: &[i32]) -> usize {
    (3..numbers.len())
        .filter(|i| numbers[i - 3] < numbers[*i])
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let numbers: Vec<i32> = lines.iter().map(|line| line.parse().unwrap()).collect();

    (solve_a(&numbers).to_string(), solve_b(&numbers).to_string())
}
