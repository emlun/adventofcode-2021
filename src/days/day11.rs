use crate::common::Solution;

fn simulate(mut map: Vec<Vec<i8>>, times: usize) -> (Vec<Vec<i8>>, usize) {
    if times == 0 {
        (map, 0)
    } else {
        for r in 0..map.len() {
            for c in 0..map.len() {
                if map[r][c] < 10 {
                    map[r][c] += 1;
                }
            }
        }
        for r in 0..map.len() {
            for c in 0..map.len() {
                flash(&mut map, r, c);
            }
        }
        let mut flashes = 0;
        for r in 0..map.len() {
            for c in 0..map.len() {
                if map[r][c] > 9 {
                    map[r][c] = 0;
                    flashes += 1;
                }
            }
        }
        let (newmap, fl) = simulate(map, times - 1);
        (newmap, flashes + fl)
    }
}

fn flash(map: &mut Vec<Vec<i8>>, r: usize, c: usize) {
    if map[r][c] == 10 {
        map[r][c] += 1;
        for rr in r.saturating_sub(1)..=(r + 1) {
            for cc in c.saturating_sub(1)..=(c + 1) {
                if rr < map.len() && cc < map[rr].len() && (rr != r || cc != c) {
                    if map[rr][cc] < 10 {
                        map[rr][cc] += 1;
                    }
                    flash(map, rr, cc);
                }
            }
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<i8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let (_, sol_a) = simulate(map, 100);

    let sol_b: usize = 0;

    (sol_a.to_string(), sol_b.to_string())
}
