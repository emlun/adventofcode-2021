use crate::common::Solution;

fn play_turn<I: Iterator<Item = u32>>(pos: u32, score: u32, die: &mut I) -> (u32, u32) {
    let dist: u32 = die.by_ref().take(3).sum();
    let new_pos = (pos + dist) % 10;
    (new_pos, score + new_pos + 1)
}

pub fn solve(lines: &[String]) -> Solution {
    let mut player1: u32 = lines[0][lines[0].len() - 1..lines[0].len()]
        .parse::<u32>()
        .unwrap()
        - 1;
    let mut player2: u32 = lines[1][lines[1].len() - 1..lines[1].len()]
        .parse::<u32>()
        .unwrap()
        - 1;

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

    let sol_a = loser * rolls;
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
