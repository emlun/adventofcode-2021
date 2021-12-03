use crate::common::Solution;

fn solve_a(lines: &[String]) -> usize {
    let mut counts = vec![0; lines[0].len()];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }
    let mut gamma: usize = 0;
    for i in 0..lines[0].len() {
        gamma <<= 1;
        if counts[i] > lines.len() / 2 {
            gamma |= 1;
        }
    }
    let epsilon: usize = (!gamma) & ((1 << lines[0].len()) - 1);
    gamma * epsilon
}

fn solve_b(lines: &[String]) -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(&lines).to_string(), solve_b(&lines).to_string())
}
