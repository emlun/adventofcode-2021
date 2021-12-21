use crate::common::Solution;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Universe {
    pos: u32,
    score: u32,
}

fn play_turn<I: Iterator<Item = u32>>(pos: u32, score: u32, die: &mut I) -> (u32, u32) {
    let dist: u32 = die.by_ref().take(3).sum();
    let new_pos = (pos + dist) % 10;
    (new_pos, score + new_pos + 1)
}

fn play_multiverse_turn(universes: HashMap<Universe, u64>) -> HashMap<Universe, u64> {
    universes
        .into_iter()
        .flat_map(|(Universe { pos, score }, num_universes)| {
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                .iter()
                .map(move |(steps, weight)| {
                    let new_pos = (pos + steps) % 10;
                    let new_score = score + new_pos + 1;
                    (
                        Universe {
                            pos: new_pos,
                            score: new_score,
                        },
                        num_universes * weight,
                    )
                })
        })
        .fold(HashMap::new(), |mut universes, (universe, weight)| {
            *universes.entry(universe).or_insert(0) += weight;
            universes
        })
}

fn simulate_multiverse(start_pos: u32) -> (Vec<u64>, Vec<u64>) {
    let mut universes: HashMap<Universe, u64> = vec![(
        Universe {
            pos: start_pos,
            score: 0,
        },
        1,
    )]
    .into_iter()
    .collect();

    let mut won_universes_by_turn = vec![0];
    let mut not_won_universes_by_turn = vec![1];

    while !universes.is_empty() {
        universes = play_multiverse_turn(universes);

        let total_universes = universes.values().sum::<u64>();

        let mut won_universes = 0;
        universes.retain(|Universe { score, .. }, weight| {
            if *score >= 21 {
                won_universes += *weight;
                false
            } else {
                true
            }
        });

        won_universes_by_turn.push(won_universes);
        not_won_universes_by_turn.push(total_universes - won_universes);
    }

    (won_universes_by_turn, not_won_universes_by_turn)
}

fn solve_a(mut player1: u32, mut player2: u32) -> u32 {
    let mut score1: u32 = 0;
    let mut score2: u32 = 0;

    let mut die = std::iter::repeat(1..=100).flatten();
    let mut rolls = 0;

    let loser = loop {
        let (tmp1, tmp2) = play_turn(player1, score1, &mut die);
        rolls += 3;
        player1 = tmp1;
        score1 = tmp2;

        if score1 >= 1000 {
            break score2;
        }

        let (tmp1, tmp2) = play_turn(player2, score2, &mut die);
        rolls += 3;
        player2 = tmp1;
        score2 = tmp2;
        if score2 >= 1000 {
            break score1;
        }
    };

    loser * rolls
}

fn solve_b(player1: u32, player2: u32) -> u64 {
    let (p1_win_universes, p1_notwin_universes) = simulate_multiverse(player1);
    let (p2_win_universes, p2_notwin_universes) = simulate_multiverse(player2);

    let p1_wins: u64 = (1..p1_win_universes.len())
        .map(|turn| p1_win_universes[turn] * p2_notwin_universes[turn - 1])
        .sum();
    let p2_wins: u64 = (0..p2_win_universes.len())
        .map(|turn| p2_win_universes[turn] * p1_notwin_universes[turn])
        .sum();

    std::cmp::max(p1_wins, p2_wins)
}

pub fn solve(lines: &[String]) -> Solution {
    let player1: u32 = lines[0][lines[0].len() - 1..lines[0].len()]
        .parse::<u32>()
        .unwrap()
        - 1;
    let player2: u32 = lines[1][lines[1].len() - 1..lines[1].len()]
        .parse::<u32>()
        .unwrap()
        - 1;

    let sol_a = solve_a(player1, player2);
    let sol_b = solve_b(player1, player2);

    (sol_a.to_string(), sol_b.to_string())
}
