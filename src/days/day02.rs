use crate::common::Solution;

fn solve_a(instructions: &[String]) -> i64 {
    let mut x: i64 = 0;
    let mut y = 0;
    for ins in instructions {
        let mut splits = ins.split_whitespace();
        match (splits.next(), splits.next()) {
            (Some("forward"), Some(dx)) => x += dx.parse::<i64>().unwrap(),
            (Some("down"), Some(dy)) => y += dy.parse::<i64>().unwrap(),
            (Some("up"), Some(dy)) => y -= dy.parse::<i64>().unwrap(),
            _ => unreachable!(),
        }
    }
    x * y
}

fn solve_b(instructions: &[String]) -> i64 {
    let mut x: i64 = 0;
    let mut y = 0;
    let mut a: i64 = 0;
    for ins in instructions {
        let mut splits = ins.split_whitespace();
        match (splits.next(), splits.next()) {
            (Some("forward"), Some(dx)) => {
                let dx = dx.parse::<i64>().unwrap();
                x += dx;
                y += a * dx;
            }
            (Some("down"), Some(dy)) => a += dy.parse::<i64>().unwrap(),
            (Some("up"), Some(dy)) => a -= dy.parse::<i64>().unwrap(),
            _ => unreachable!(),
        }
    }
    x * y
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
