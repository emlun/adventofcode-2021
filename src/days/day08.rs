use crate::common::Solution;
use std::collections::HashMap;

fn analyze_entry(unidentified: Vec<u8>, output: &[u8]) -> u64 {
    let (mut identified, twothreefive, zerosixnine): (HashMap<u8, u8>, Vec<u8>, Vec<u8>) =
        unidentified.into_iter().fold(
            (HashMap::new(), Vec::new(), Vec::new()),
            |(mut identified, mut twothreefive, mut zerosixnine), next| {
                match next.count_ones() {
                    2 => {
                        identified.insert(next, 1);
                    }
                    3 => {
                        identified.insert(next, 7);
                    }
                    4 => {
                        identified.insert(next, 4);
                    }
                    7 => {
                        identified.insert(next, 8);
                    }
                    5 => {
                        twothreefive.push(next);
                    }
                    6 => {
                        zerosixnine.push(next);
                    }
                    _ => unreachable!(),
                };
                (identified, twothreefive, zerosixnine)
            },
        );

    let one: u8 = *identified
        .iter()
        .find(|(_, v)| **v == 1)
        .map(|(k, _)| k)
        .unwrap();

    let (three, twofive): (Vec<u8>, Vec<u8>) =
        twothreefive.into_iter().partition(|ttf| ttf & one == one);
    let three = three[0];

    let (nine, zerosix): (Vec<u8>, Vec<u8>) = zerosixnine
        .into_iter()
        .partition(|zsn| zsn & three == three);
    let nine = nine[0];

    let (five, two): (Vec<u8>, Vec<u8>) = twofive.into_iter().partition(|tf| tf & nine == *tf);
    let five = five[0];

    let (six, zero): (Vec<u8>, Vec<u8>) = zerosix.into_iter().partition(|zs| zs & five == five);

    identified.insert(zero[0], 0);
    identified.insert(two[0], 2);
    identified.insert(three, 3);
    identified.insert(five, 5);
    identified.insert(six[0], 6);
    identified.insert(nine, 9);

    output.iter().fold(0, |num, digit| {
        num * 10 + u64::from(*identified.get(digit).unwrap())
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let entries: Vec<(Vec<u8>, Vec<u8>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split('|').map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|s| {
                        s.chars()
                            .map(|c| match c {
                                'a' => 0x01,
                                'b' => 0x02,
                                'c' => 0x04,
                                'd' => 0x08,
                                'e' => 0x10,
                                'f' => 0x20,
                                'g' => 0x40,
                                _ => unreachable!(),
                            })
                            .sum()
                    })
                    .collect()
            });

            (halves.next().unwrap(), halves.next().unwrap())
        })
        .collect();

    let sol_a = entries
        .iter()
        .flat_map(|(_, o)| o)
        .filter(|o| matches!(o.count_ones(), 2 | 4 | 3 | 7))
        .count();
    let sol_b: u64 = entries.into_iter().map(|(i, o)| analyze_entry(i, &o)).sum();

    (sol_a.to_string(), sol_b.to_string())
}
