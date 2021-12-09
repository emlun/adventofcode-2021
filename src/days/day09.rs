use crate::common::Solution;
use std::collections::VecDeque;

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Vec<Vec<i8>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    map.insert(0, vec![9; map[0].len()]);
    map.push(vec![9; map[0].len()]);
    for row in &mut map {
        row.insert(0, 9);
        row.push(9);
    }

    let low_points: Vec<(usize, usize)> = (1..map[0].len() - 1)
        .flat_map(|x| (1..map.len() - 1).map(move |y| (x, y)))
        .filter(|(x, y)| {
            [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)]
                .iter()
                .all(|(nx, ny)| map[*ny][*nx] > map[*y][*x])
        })
        .collect();

    let mut floodmap = map.clone();
    let mut basins: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut size = 0;
            queue.push_back((*x, *y));
            while let Some((x, y)) = queue.pop_front() {
                if floodmap[y][x] >= 0 && floodmap[y][x] < 9 {
                    size += 1;
                    floodmap[y][x] = -1;
                    queue.push_back((x - 1, y));
                    queue.push_back((x + 1, y));
                    queue.push_back((x, y - 1));
                    queue.push_back((x, y + 1));
                }
            }
            size
        })
        .collect();
    basins.sort();

    let sol_a: i32 = low_points.iter().map(|(x, y)| 1 + map[*y][*x] as i32).sum();
    let sol_b: usize = basins[(basins.len() - 3)..basins.len()].iter().product();

    (sol_a.to_string(), sol_b.to_string())
}
