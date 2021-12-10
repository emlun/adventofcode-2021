use crate::common::Solution;

#[derive(Debug)]
enum Chunk {
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn parse_chunk<I: Iterator<Item = char>>(input: &mut I) -> Chunk {
    let mut opens = Vec::new();
    while let Some(next) = input.next() {
        match next {
            '(' | '[' | '{' | '<' => opens.push(next),
            ')' | ']' | '}' | '>' => match (opens.pop(), next) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
                (_, close) => {
                    return Chunk::Corrupt(close);
                }
            },
            _ => unreachable!(),
        }
    }
    Chunk::Incomplete(opens)
}

pub fn solve(lines: &[String]) -> Solution {
    let chunks: Vec<Chunk> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_chunk(&mut l.chars()))
        .collect();

    let sol_a: usize = chunks
        .iter()
        .flat_map(|c| match c {
            Chunk::Corrupt(')') => Some(3),
            Chunk::Corrupt(']') => Some(57),
            Chunk::Corrupt('}') => Some(1197),
            Chunk::Corrupt('>') => Some(25137),
            _ => None,
        })
        .sum();

    let mut completion_scores: Vec<usize> = chunks
        .into_iter()
        .flat_map(|c| match c {
            Chunk::Incomplete(opens) => Some(opens.into_iter().rev().fold(0, |score, chr| {
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
    completion_scores.sort();
    let sol_b = completion_scores[completion_scores.len() / 2];

    (sol_a.to_string(), sol_b.to_string())
}
