use crate::common::Solution;

pub fn solve_a(bitlen: usize, nums: &[usize]) -> (usize, usize) {
    let threshold = nums.len() / 2 + (nums.len() % 2);
    let gamma: usize = (0..bitlen)
        .map(|i| {
            let mask: usize = 1 << i;
            if nums.iter().filter(|n| **n & mask != 0).count() >= threshold {
                1 << i
            } else {
                0
            }
        })
        .sum();
    let epsilon: usize = (!gamma) & ((1 << bitlen) - 1);

    (gamma, epsilon)
}

pub fn solve(lines: &[String]) -> Solution {
    let bitlen = lines[0].len();
    let input_nums: Vec<usize> = lines
        .iter()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let (gamma, epsilon) = solve_a(bitlen, &input_nums);

    let oxy_candidates: Vec<usize> =
        (0..bitlen)
            .rev()
            .map(|i| 1 << i)
            .fold(input_nums.clone(), |cands, mask| {
                if cands.len() == 1 {
                    cands
                } else {
                    let (g, _) = solve_a(bitlen, &cands);
                    cands
                        .into_iter()
                        .filter(|o| (o & mask) == (g & mask))
                        .collect()
                }
            });
    let oxy = oxy_candidates.into_iter().next().unwrap();

    let co2_candidates: Vec<usize> =
        (0..bitlen)
            .rev()
            .map(|i| 1 << i)
            .fold(input_nums, |cands, mask| {
                if cands.len() == 1 {
                    cands
                } else {
                    let (_, e) = solve_a(bitlen, &cands);
                    cands
                        .into_iter()
                        .filter(|o| (o & mask) == (e & mask))
                        .collect()
                }
            });
    let co2 = co2_candidates.into_iter().next().unwrap();

    ((gamma * epsilon).to_string(), (oxy * co2).to_string())
}
