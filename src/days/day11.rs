use crate::common::Solution;

const SIZE: usize = 10;

fn simulate(mut map: Vec<Vec<i8>>) -> (Vec<Vec<i8>>, usize) {
    for r in 0..SIZE {
        for c in 0..SIZE {
            if map[r][c] < 10 {
                map[r][c] += 1;
            }
            flash(&mut map, r, c);
        }
    }
    let mut flashes = 0;
    for r in 0..SIZE {
        for c in 0..SIZE {
            if map[r][c] > 9 {
                map[r][c] = 0;
                flashes += 1;
            }
        }
    }
    (map, flashes)
}

fn flash(map: &mut Vec<Vec<i8>>, r: usize, c: usize) {
    if map[r][c] == 10 {
        map[r][c] += 1;
        for rr in r.saturating_sub(1)..=std::cmp::min(r + 1, SIZE - 1) {
            for cc in c.saturating_sub(1)..=std::cmp::min(c + 1, SIZE - 1) {
                if rr != r || cc != c {
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
    let mut map: Vec<Vec<i8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let mut sol_a = 0;
    let mut sol_b = None;
    for t in 1..=100 {
        let (m, f) = simulate(map);
        map = m;
        if f == SIZE * SIZE {
            sol_b = sol_b.or(Some(t));
        }
        sol_a += f;
    }
    for t in 101.. {
        if sol_b.is_some() {
            break;
        }
        let (m, f) = simulate(map);
        map = m;
        if f == SIZE * SIZE {
            sol_b = sol_b.or(Some(t));
        }
    }
    let sol_b = sol_b.unwrap();

    (sol_a.to_string(), sol_b.to_string())
}
