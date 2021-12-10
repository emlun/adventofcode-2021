use crate::common::Solution;

#[derive(Debug)]
enum Chunk {
    Valid(char, Vec<Chunk>),
    Incomplete,
    Corrupt(char),
}
use Chunk::Corrupt;
use Chunk::Incomplete;
use Chunk::Valid;

fn parse_chunk<I: Iterator<Item = char>>(input: &mut std::iter::Peekable<I>) -> Chunk {
    if let Some(open) = input.next() {
        match open {
            '(' | '[' | '{' | '<' => {
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
                        Some(_) => match parse_chunk(input) {
                            subchunk @ Valid(_, _) => {
                                subchunks.push(subchunk);
                            }
                            i @ Incomplete => break i,
                            c @ Corrupt(_) => break c,
                        },
                        None => break Incomplete,
                    }
                }
            }
            other => Corrupt(other),
        }
    } else {
        Incomplete
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let chunks: Vec<Chunk> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse_chunk(&mut l.chars().peekable()))
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

    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
