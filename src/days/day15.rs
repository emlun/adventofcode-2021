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

fn mod1(x: usize, n: usize) -> usize {
    let y = x % n;
    if y == 0 {
        n
    } else {
        y
    }
}

fn search(map: &[Vec<u8>], extend: bool) -> usize {
    let mut queue = BinaryHeap::new();
    let (base_height, base_width) = (map.len(), map[0].len());
    let (height, width) = if extend {
        (base_height * 5, base_width * 5)
    } else {
        (base_height, base_width)
    };
    let goal = (width - 1, height - 1);
    let mut visited = vec![vec![false; width]; height];
    queue.push(Path {
        pos: (0, 0),
        risk: 0,
    });
    while let Some(Path { pos, risk }) = queue.pop() {
        let (x, y) = pos;
        if pos == goal {
            return risk;
        } else if !visited[y][x] {
            visited[y][x] = true;
            queue.extend(
                [
                    x.checked_sub(1).map(|xx| (xx, y)),
                    Some((x + 1, y)).filter(|(xx, _)| *xx < width),
                    y.checked_sub(1).map(|yy| (x, yy)),
                    Some((x, y + 1)).filter(|(_, yy)| *yy < height),
                ]
                .iter()
                .flatten()
                .copied()
                .filter(|(xx, yy)| !visited[*yy][*xx])
                .map(|(xx, yy)| Path {
                    pos: (xx, yy),
                    risk: risk
                        + mod1(
                            usize::from(map[yy % base_height][xx % base_width])
                                + yy / base_height
                                + xx / base_width,
                            9,
                        ),
                }),
            );
        }
    }
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<u8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let sol_a = search(&map, false);
    let sol_b = search(&map, true);

    (sol_a.to_string(), sol_b.to_string())
}
