use crate::common::Solution;

#[derive(Debug)]
enum Chunk {
    Valid(char, Vec<Chunk>),
    Incomplete(char, Vec<Chunk>),
    Corrupt(char),
}
use Chunk::Corrupt;
use Chunk::Incomplete;
use Chunk::Valid;

fn parse_chunk<I: Iterator<Item = char>>(input: &mut std::iter::Peekable<I>) -> Option<Chunk> {
    match input.next() {
        Some(open @ ('(' | '[' | '{' | '<')) => Some({
            let mut subchunks = Vec::new();
            loop {
                match input.peek() {
                    Some(')' | ']' | '}' | '>') => {
                        let close = input.next().unwrap();
                        match (open, close) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                                break Valid(open, subchunks)
                            }
                            _ => break Corrupt(close),
                        };
                    }
                    _ => match parse_chunk(input) {
                        Some(subchunk @ Valid(..)) => {
                            subchunks.push(subchunk);
                        }
                        Some(i @ Incomplete(..)) => {
                            subchunks.push(i);
                        }
                        Some(c @ Corrupt(..)) => break c,
                        None => break Incomplete(open, subchunks),
                    },
                }
            }
        }),
        Some(other) => Some(Corrupt(other)),
        None => None,
    }
}

fn complete_chunk(chunk: &Chunk) -> Vec<char> {
    fn recurse(chunk: &Chunk, output: &mut Vec<char>) {
        match chunk {
            Incomplete(open, subchunks) => {
                for sub in subchunks {
                    recurse(sub, output);
                }
                output.push(match open {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!(),
                });
            }
            _ => {}
        }
    }
    let mut output = Vec::new();
    recurse(chunk, &mut output);
    output
}

pub fn solve(lines: &[String]) -> Solution {
    let chunks: Vec<Chunk> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .flat_map(|l| parse_chunk(&mut l.chars().peekable()))
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
        .iter()
        .filter(|c| matches!(c, Incomplete(..)))
        .map(|c| {
            complete_chunk(c).into_iter().fold(0, |score, chr| {
                score * 5
                    + match chr {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();
    completion_scores.sort();
    let sol_b = completion_scores[completion_scores.len() / 2];

    (sol_a.to_string(), sol_b.to_string())
}
