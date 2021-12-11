use crate::common::Solution;

fn simulate(mut map: Vec<Vec<i8>>, times: usize) -> (Vec<Vec<i8>>, usize, Option<usize>) {
    if times == 0 {
        (map, 0, None)
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
        let allflash = if flashes == map.len() * map.len() {
            Some(times)
        } else {
            None
        };
        let (newmap, fl, allflash_later) = simulate(map, times - 1);
        (newmap, flashes + fl, allflash.or(allflash_later))
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

    let (mut map, sol_a, mut sol_b) = simulate(map, 100);
    let mut time = 101;
    let sol_b = loop {
        if sol_b.is_some() {
            break time - sol_b.unwrap();
        } else {
            let (mp, _, sb) = simulate(map, 100);
            map = mp;
            sol_b = sb;
            time += 100;
        }
    };

    (sol_a.to_string(), sol_b.to_string())
}
