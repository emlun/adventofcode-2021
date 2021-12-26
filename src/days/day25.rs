use crate::common::Solution;

#[derive(Clone, Eq, PartialEq)]
enum Cucumber {
    Empty,
    West,
    South,
}
use Cucumber::*;

fn step(map: &Vec<Vec<Cucumber>>) -> Option<Vec<Vec<Cucumber>>> {
    let mut west_result = map.clone();
    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                Empty | South => {}
                West => {
                    let cc = (c + 1) % row.len();
                    if map[r][cc] == Empty {
                        west_result[r][c] = Empty;
                        west_result[r][cc] = West;
                    }
                }
            }
        }
    }

    let mut south_result = west_result.clone();

    for (r, row) in west_result.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                Empty | West => {}
                South => {
                    let rr = (r + 1) % west_result.len();
                    if west_result[rr][c] == Empty {
                        south_result[r][c] = Empty;
                        south_result[rr][c] = South;
                    }
                }
            }
        }
    }

    Some(south_result).filter(|r| r != map)
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<Cucumber>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '>' => West,
                    'v' => South,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let sol_a = std::iter::successors(Some(map), step)
        .enumerate()
        .last()
        .unwrap()
        .0
        + 1;
    let sol_b = "";

    (sol_a.to_string(), sol_b.to_string())
}
