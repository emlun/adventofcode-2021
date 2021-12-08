use crate::common::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

fn analyze_entry(unidentified: HashSet<String>, output: &[String]) -> u64 {
    let (mut identified, mut twothreefive, mut zerosixnine): (
        HashMap<String, u8>,
        HashSet<String>,
        HashSet<String>,
    ) = unidentified.into_iter().fold(
        (HashMap::new(), HashSet::new(), HashSet::new()),
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
                    twothreefive.insert(next);
                }
                6 => {
                    zerosixnine.insert(next);
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
    let five: String = twothreefive
        .iter()
        .find(|s| s.contains(five_discriminator))
        .unwrap()
        .clone();
    identified.insert(twothreefive.take(&five).unwrap(), 5);

    let two_discriminator: char = twothreefive
        .iter()
        .flat_map(|s| s.chars())
        .find(|c| {
            twothreefive
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
    let (two, three): (String, String) = twothreefive
        .into_iter()
        .partition(|tt| tt.contains(two_discriminator));
    identified.insert(two, 2);
    identified.insert(three, 3);

    let nine: String = zerosixnine
        .iter()
        .find(|zsn| !zsn.contains(two_discriminator))
        .unwrap()
        .clone();
    identified.insert(zerosixnine.take(&nine).unwrap(), 9);

    let zero_discriminator: char = nine
        .chars()
        .find(|n| {
            zerosixnine
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
    let (zero, six): (String, String) = zerosixnine
        .into_iter()
        .partition(|zs| zs.chars().any(|c| c == zero_discriminator));
    identified.insert(zero, 0);
    identified.insert(six, 6);

    output.into_iter().fold(0, |num, digit| {
        num * 10 + u64::from(*identified.get(digit).unwrap())
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let entries: Vec<(HashSet<String>, Vec<String>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut halves = l.split('|').map(|s| s.trim());
            (
                halves
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| {
                        let mut v = s.chars().collect::<Vec<char>>();
                        v.sort();
                        v.into_iter().collect()
                    })
                    .collect(),
                halves
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| {
                        let mut v = s.chars().collect::<Vec<char>>();
                        v.sort();
                        v.into_iter().collect()
                    })
                    .collect(),
            )
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
