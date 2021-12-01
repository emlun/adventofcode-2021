use crate::common::Solution;

fn solve_a(numbers: &[i32]) -> usize {
    let mut c = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] < numbers[i] {
            c += 1;
        }
    }
    c
}

fn solve_b(numbers: &[i32]) -> i32 {
    unreachable!();
}

pub fn solve(lines: &[String]) -> Solution {
    let mut numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    (solve_a(&numbers).to_string(), 0.to_string())
}
