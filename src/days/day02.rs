use crate::common::Solution;

fn solve_a(instructions: &[String]) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for ins in instructions {
        let mut splits = ins.split_whitespace();
        let word = splits.next().unwrap();
        let arg: i64 = splits.next().unwrap().parse().unwrap();
        match word {
            "forward" => x += arg,
            "down" => y += arg,
            "up" => y -= arg,
            _ => unreachable!(),
        }
    }
    x * y
}

fn solve_b(instructions: &[String]) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut a: i64 = 0;
    for ins in instructions {
        let mut splits = ins.split_whitespace();
        let word = splits.next().unwrap();
        let arg: i64 = splits.next().unwrap().parse().unwrap();
        match word {
            "forward" => {
                x += arg;
                y += a * arg;
            }
            "down" => a += arg,
            "up" => a -= arg,
            _ => unreachable!(),
        }
    }
    x * y
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
