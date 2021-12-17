use crate::common::Solution;

pub fn sign(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i / i.abs()
    }
}

fn xt(t: i64, vx0: i64) -> i64 {
    (vx0 * (vx0 + 1) / 2) - std::cmp::max(0, t * (t + 1) / 2)
}

fn yt(t: i64, vy0: i64) -> i64 {
    t * vy0 - ((t - 1) * t) / 2
}

fn simulate(minx: i64, maxx: i64, miny: i64, maxy: i64, vx0: i64, vy0: i64) -> Option<i64> {
    // let t_at_ymax = std::cmp::max(0, vy0);
    // let ymax = t_at_ymax * vy0 + (t_at_ymax - t_at_ymax * t_at_ymax) / 2;

    // let t_before_impact = ((((2 * vy0 + 1) / 2) as f64)
    //     + ((((2 * vy0 + 1) / 2).pow(2) - 2 * maxy) as f64).sqrt())
    // .floor()
    // .trunc() as i64;
    // let x_before_impact = xt(t_before_impact, vx0);
    // let y_before_impact = yt(t_before_impact, vy0);
    // let vx_before_impact = std::cmp::max(0, vx0 - t_before_impact);
    // let vy_before_impact = vy0 - t_before_impact;
    // dbg!(
    //     vx0,
    //     vy0,
    //     t_before_impact,
    //     x_before_impact,
    //     y_before_impact,
    //     vx_before_impact,
    //     vy_before_impact
    // );

    // let mut x = x_before_impact + vx_before_impact;
    // let mut y = y_before_impact + vy_before_impact;
    // let mut vx = vx_before_impact - sign(vx_before_impact);
    // let mut vy = vy_before_impact - 1;

    let mut x = 0;
    let mut y = 0;
    let mut vx = vx0;
    let mut vy = vy0;
    let mut ymax = 0;
    while y >= miny {
        // dbg!(x, y, vx, vy);
        if x >= minx && x <= maxx && y >= miny && y <= maxy {
            return Some(ymax);
        }

        x += vx;
        y += vy;
        vx -= sign(vx);
        vy -= 1;
        ymax = std::cmp::max(ymax, y);
    }
    None
}

fn solve_a(minx: i64, maxx: i64, miny: i64, maxy: i64) -> i64 {
    let mut top_topy = 0;

    // min_vx^2 + min_vx - minx * 2 = 0
    // x^2 + x - c = 0

    let vx_min = ((-0.5) + (0.25 + (minx as f64) * 2.0).sqrt()).ceil() as i64;
    let vx_max = ((-0.5) + (0.25 + (maxx as f64) * 2.0).sqrt()).floor() as i64;
    // dbg!(vx_min, vx_max);

    for topy in (0..miny.abs()).flat_map(|absvy| {
        (vx_min..maxx)
            .map(move |vx| simulate(minx, maxx, miny, maxy, vx, absvy))
            .skip_while(|result| result.is_none())
            .take_while(|result| result.is_some())
            .flatten()
    }) {
        if topy < top_topy {
            break;
        } else {
            top_topy = topy;
        }
    }
    top_topy
}

pub fn solve(lines: &[String]) -> Solution {
    let mut target_halves = lines
        .iter()
        .find(|l| !l.is_empty())
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(", ");
    let mut x_halves = target_halves
        .next()
        .unwrap()
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .split("..");
    let minx: i64 = x_halves.next().unwrap().parse().unwrap();
    let maxx: i64 = x_halves.next().unwrap().parse().unwrap();
    let mut y_halves = target_halves
        .next()
        .unwrap()
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .split("..");
    let miny: i64 = y_halves.next().unwrap().parse().unwrap();
    let maxy: i64 = y_halves.next().unwrap().parse().unwrap();

    // dbg!(minx, maxx, miny, maxy);

    let sol_a = solve_a(minx, maxx, miny, maxy);
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
