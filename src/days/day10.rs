use crate::common::Solution;

#[derive(Debug)]
enum Chunk {
    Incomplete(Vec<char>),
    Corrupt(char),
}
use Chunk::Corrupt;
use Chunk::Incomplete;

fn parse_chunk(input: &str) -> Chunk {
    let mut opens = Vec::new();
    for next in input.chars() {
        match next {
            '(' | '[' | '{' | '<' => opens.push(next),
            ')' | ']' | '}' | '>' => match (opens.pop(), next) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
                (_, close) => {
                    return Corrupt(close);
                }
            },
            _ => unreachable!(),
        }
    }
    Incomplete(opens)
}

pub fn solve(lines: &[String]) -> Solution {
    let chunks: Vec<Chunk> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_chunk(l))
        .collect();

    let sol_a: usize = chunks
        .iter()
        .flat_map(|c| match c {
            Corrupt(')') => Some(3),
            Corrupt(']') => Some(57),
            Corrupt('}') => Some(1197),
            Corrupt('>') => Some(25137),
            _ => None,
        })
        .sum();

    let mut completion_scores: Vec<usize> = chunks
        .into_iter()
        .flat_map(|c| match c {
            Incomplete(opens) => Some(opens.into_iter().rev().fold(0, |score, chr| {
                score * 5
                    + match chr {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            })),
            _ => None,
        })
        .collect();
    completion_scores.sort_unstable();
    let sol_b = completion_scores[completion_scores.len() / 2];

    (sol_a.to_string(), sol_b.to_string())
}
