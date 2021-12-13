use crate::common::Solution;
use std::collections::HashSet;

fn format_map(map: &HashSet<(usize, usize)>) -> String {
    let maxx = *map.iter().map(|(x, _)| x).max().unwrap();
    (0..=(*map.iter().map(|(_, y)| y).max().unwrap()))
        .map(|r| {
            (0..=maxx)
                .map(|c| if map.contains(&(c, r)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn fold_map(map: HashSet<(usize, usize)>, fold_line: &str) -> HashSet<(usize, usize)> {
    let mut fold_halves = fold_line.split('=');
    let fold_words: &str = fold_halves.next().unwrap();
    let fold_coord: usize = fold_halves.next().unwrap().parse().unwrap();
    let is_x = fold_words.ends_with('x');
    map.into_iter()
        .map(|(x, y)| {
            if is_x && x > fold_coord {
                (2 * fold_coord - x, y)
            } else if !is_x && y > fold_coord {
                (x, 2 * fold_coord - y)
            } else {
                (x, y)
            }
        })
        .collect()
}

pub fn solve(lines: &[String]) -> Solution {
    let (fold_lines, coord_lines): (Vec<&String>, Vec<&String>) = lines
        .iter()
        .filter(|l| !l.is_empty())
        .partition(|l| l.starts_with("fold"));

    let map: HashSet<(usize, usize)> = coord_lines
        .into_iter()
        .map(|l| {
            let mut halves = l.split(',');
            (
                halves.next().unwrap().parse().unwrap(),
                halves.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut folds = fold_lines.into_iter();

    let folded_once: HashSet<(usize, usize)> = fold_map(map, folds.next().unwrap());
    let sol_a = folded_once.len();

    let folded_fully: HashSet<(usize, usize)> =
        folds.fold(folded_once, |map, next_fold| fold_map(map, next_fold));

    let sol_b = format_map(&folded_fully);

    (sol_a.to_string(), format!("\n{}", sol_b))
}
