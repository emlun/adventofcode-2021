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
    let mut minmap = vec![vec![0; width]; height];
    queue.push(Path {
        pos: (0, 0),
        risk: 0,
    });
    while let Some(Path { pos, risk }) = queue.pop() {
        let (x, y) = pos;
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
    unreachable!()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<u8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let sol_a = search(&map);
    let sol_b = search(
        &std::iter::repeat(map)
            .take(5)
            .enumerate()
            .flat_map(|(ri, rows)| rows.into_iter().map(move |row| (ri, row)))
            .map(|(ri, row)| {
                std::iter::repeat(row)
                    .take(5)
                    .enumerate()
                    .flat_map(|(ci, row)| row.into_iter().map(move |cell| (ci, cell)))
                    .map(|(ci, cell)| {
                        let newcell =
                            (cell + u8::try_from(ri).unwrap() + u8::try_from(ci).unwrap()) % 9;
                        if newcell == 0 {
                            9
                        } else {
                            newcell
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<u8>>>(),
    );

    (sol_a.to_string(), sol_b.to_string())
}
