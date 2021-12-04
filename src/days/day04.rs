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

fn play(side: usize, draws: &[usize], mut boards: Vec<Board>) -> (usize, usize) {
    let mut scores = vec![None; boards.len()];
    let mut first_score = None;

    for drawn in draws {
        for (board_i, board) in boards.iter_mut().enumerate() {
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
                        .sum::<usize>()
                        * drawn;

                    if first_score.is_none() {
                        first_score = Some(score);
                    }
                    if scores[board_i].is_none() {
                        scores[board_i] = Some(score);
                        if scores.iter().all(|s| s.is_some()) {
                            return (first_score.unwrap(), score);
                        }
                    }
                }
            }
        }
    }
    unreachable!();
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
    let (first, last) = play(side, &draws, boards);
    (first.to_string(), last.to_string())
}
