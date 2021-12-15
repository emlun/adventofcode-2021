use crate::common::Solution;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Path {
    pos: (usize, usize),
    risk: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search(map: &[Vec<u8>]) -> usize {
    let mut queue = BinaryHeap::new();
    let height = map.len();
    let width = map[0].len();
    let goal = (width - 1, height - 1);
    // let (goalx, goaly) = goal;
    let mut minmap = vec![vec![0; width]; height];
    queue.push(Path {
        pos: (0, 0),
        risk: 0,
    });
    while let Some(Path { pos, risk }) = queue.pop() {
        let (x, y) = pos;
        // if minmap[goaly][goalx] > 0 && minmap[goaly][goalx] <= risk {
        // break;
        if pos == goal {
            return risk;
        } else if minmap[y][x] == 0 || risk < minmap[y][x] {
            minmap[y][x] = risk;
            queue.extend(
                [
                    (x.saturating_sub(1), y),
                    (x + 1, y),
                    (x, y.saturating_sub(1)),
                    (x, y + 1),
                ]
                .iter()
                .copied()
                .filter(|(xx, yy)| (*xx != x || *yy != y) && *xx < width && *yy < height)
                .filter(|(xx, yy)| minmap[*yy][*xx] == 0)
                .map(|(xx, yy)| Path {
                    pos: (xx, yy),
                    risk: risk + usize::from(map[yy][xx]),
                }),
            );
        }
    }
    // minmap[goaly][goalx]
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Vec<Vec<u8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut sol_a = search(&map);
    let mut sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
