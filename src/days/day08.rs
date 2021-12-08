use crate::common::Solution;
use std::collections::HashMap;

fn analyze_entry(unidentified: Vec<String>, output: &[String]) -> u64 {
    let (mut identified, twothreefive, zerosixnine): (
        HashMap<String, u8>,
        Vec<String>,
        Vec<String>,
    ) = unidentified.into_iter().fold(
        (HashMap::new(), Vec::new(), Vec::new()),
        |(mut identified, mut twothreefive, mut zerosixnine), next| {
            match next.len() {
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

    let five_discriminator: char = twothreefive
        .iter()
        .flat_map(|s| s.chars())
        .find(|c| {
            twothreefive.iter().filter(|s| s.contains(*c)).count() == 1
                && zerosixnine.iter().filter(|s| s.contains(*c)).count() == 3
        })
        .unwrap();
    let (five, twothree): (Vec<String>, Vec<String>) = twothreefive
        .into_iter()
        .partition(|s| s.contains(five_discriminator));

    let two_discriminator: char = twothree
        .iter()
        .flat_map(|s| s.chars())
        .find(|c| {
            twothree
                .iter()
                .flat_map(|s| s.chars())
                .filter(|c2| c2 == c)
                .count()
                == 1
                && zerosixnine
                    .iter()
                    .flat_map(|s| s.chars())
                    .filter(|c2| c2 == c)
                    .count()
                    == 2
        })
        .unwrap();
    let (two, three): (String, String) = twothree
        .into_iter()
        .partition(|tt| tt.contains(two_discriminator));

    let (nine, zerosix): (Vec<String>, Vec<String>) = zerosixnine
        .into_iter()
        .partition(|zsn| !zsn.contains(two_discriminator));

    let zero_discriminator: char = nine[0]
        .chars()
        .find(|n| {
            zerosix
                .iter()
                .flat_map(|s| s.chars())
                .filter(|c| c == n)
                .count()
                == 1
                && identified
                    .iter()
                    .filter(|(_, v)| **v == 7)
                    .flat_map(|(k, _)| k.chars())
                    .filter(|c| c == n)
                    .count()
                    == 1
        })
        .unwrap();
    let (zero, six): (String, String) = zerosix
        .into_iter()
        .partition(|zs| zs.chars().any(|c| c == zero_discriminator));

    identified.insert(zero, 0);
    identified.insert(two, 2);
    identified.insert(three, 3);
    identified.insert(five.into_iter().next().unwrap(), 5);
    identified.insert(six, 6);
    identified.insert(nine.into_iter().next().unwrap(), 9);

    output.into_iter().fold(0, |num, digit| {
        num * 10 + u64::from(*identified.get(digit).unwrap())
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let entries: Vec<(Vec<String>, Vec<String>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split('|').map(|s| {
                s.trim()
                    .split_whitespace()
                    .map(|s| {
                        let mut v: Vec<char> = s.chars().collect();
                        v.sort();
                        v.into_iter().collect()
                    })
                    .collect()
            });

            (halves.next().unwrap(), halves.next().unwrap())
        })
        .collect();

    let sol_a = entries
        .iter()
        .flat_map(|(_, o)| o)
        .filter(|o| o.len() == 2 || o.len() == 4 || o.len() == 3 || o.len() == 7)
        .count();
    let sol_b: u64 = entries.into_iter().map(|(i, o)| analyze_entry(i, &o)).sum();

    (sol_a.to_string(), sol_b.to_string())
}
