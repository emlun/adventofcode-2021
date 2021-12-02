use crate::common::Solution;

fn solve_a(instructions: &[String]) -> i32 {
    let mut x: i32 = 0;
    let mut y = 0;
    for ins in instructions {
        let mut splits = ins.split_whitespace();
        match (splits.next(), splits.next()) {
            (Some("forward"), Some(dx)) => x += dx.parse::<i32>().unwrap(),
            (Some("down"), Some(dy)) => y += dy.parse::<i32>().unwrap(),
            (Some("up"), Some(dy)) => y -= dy.parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
    }
    x * y
}

fn solve_b() -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b().to_string())
}
