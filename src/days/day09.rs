use crate::common::Solution;
use std::collections::HashMap;

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Vec<Vec<i8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    map.insert(0, vec![10; map[0].len()]);
    map.push(vec![10; map[0].len()]);
    for row in &mut map {
        row.insert(0, 10);
        row.push(10);
    }

    let sol_a: i32 = (1..map[0].len() - 1)
        .flat_map(|x| (1..map.len() - 1).map(move |y| (x, y)))
        .map(|(x, y)| {
            if [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .all(|(nx, ny)| map[*ny][*nx] > map[y][x])
            {
                1 + map[y][x] as i32
            } else {
                0
            }
        })
        .sum();
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
