use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let mut x: i64 = 0;
    let mut ay: i64 = 0;
    let mut by: i64 = 0;
    let mut aim: i64 = 0;
    for ins in lines {
        let mut splits = ins.split_whitespace();
        let word = splits.next().unwrap();
        let arg: i64 = splits.next().unwrap().parse().unwrap();
        match word {
            "forward" => {
                x += arg;
                by += aim * arg;
            }
            "down" => {
                ay += arg;
                aim += arg;
            }
            "up" => {
                ay -= arg;
                aim -= arg;
            }
            _ => unreachable!(),
        }
    }
    ((x * ay).to_string(), (x * by).to_string())
}
