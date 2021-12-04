use crate::common::Solution;
use std::collections::HashSet;

struct Board {
    tiles: Vec<Option<(usize, usize)>>,
    rows: Vec<Vec<(usize, usize)>>,
    cols: Vec<Vec<(usize, usize)>>,
}
impl Board {
    fn new(board: Vec<Option<(usize, usize)>>, side: usize) -> Self {
        Self {
            tiles: board,
            rows: vec![vec![]; side],
            cols: vec![vec![]; side],
        }
    }
}

fn solve_a(side: usize, draws: &[usize], mut boards: Vec<Board>) -> usize {
    for drawn in draws {
        for board in boards.iter_mut() {
            if let Some(pos) = board.tiles[*drawn] {
                let (r, c) = pos;
                board.rows[r].push(pos);
                board.cols[c].push(pos);

                if board.rows[r].len() == side || board.cols[c].len() == side {
                    let marked: HashSet<(usize, usize)> =
                        board.rows.iter().flatten().copied().collect();
                    let score: usize = board
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, pos)| pos.is_some())
                        .filter(|(_, pos)| !marked.contains(&pos.unwrap()))
                        .map(|(i, _)| i)
                        .sum();
                    return score * drawn;
                }
            }
        }
    }
    0
}

fn solve_b(lines: &[String]) -> usize {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    let draws: Vec<usize> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let maxnum: usize = *draws.iter().max().unwrap();
    let side = lines[2].split_whitespace().count();
    let (boards, _, _): (Vec<Board>, Vec<Option<(usize, usize)>>, usize) = lines[2..]
        .iter()
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split_whitespace())
        .map(|s| s.parse::<usize>().unwrap())
        .fold(
            (vec![], vec![None; maxnum + 1], 0),
            |(mut boards, mut board, i), cell| {
                let r = i / side;
                let c = i % side;
                board[cell] = Some((r, c));
                if r == side - 1 && c == side - 1 {
                    boards.push(Board::new(board, side));
                    (boards, vec![None; maxnum + 1], 0)
                } else {
                    (boards, board, i + 1)
                }
            },
        );
    (
        solve_a(side, &draws, boards).to_string(),
        solve_b(lines).to_string(),
    )
}
