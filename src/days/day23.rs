use crate::common::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Point = (usize, usize);

#[derive(Eq, PartialEq)]
enum Tile {
    Floor,
    Destination(u32),
    Wall,
}

use Tile::{Destination, Floor, Wall};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Player {
    typ: u32,
    cost: usize,
    pos: Point,
    moves: usize,
}

impl Player {
    fn finished(&self) -> bool {
        let (x, y) = self.pos;
        y >= 2 && x == (self.typ as usize) * 2 + 3
    }

    fn can_move(&self) -> bool {
        self.moves < 2
    }

    fn destination_x(&self) -> usize {
        (self.typ as usize) * 2 + 3
    }

    fn available_moves(&self, state: &State) -> Vec<(Point, usize)> {
        let mut moves: Vec<(Point, usize)> = Vec::new();

        let (x, y) = self.pos;

        if y >= 2 {
            if let Some(hallway_y) = (1..y)
                .rev()
                .take_while(|ny| state.players.iter().all(|p| p.pos != (x, *ny)))
                .find(|ny| *ny == 1)
            {
                for hallway_x in ((x + 1)..12)
                    .take_while(|nx| state.players.iter().all(|p| p.pos != (*nx, hallway_y)))
                    .filter(|nx| nx != &3 && nx != &5 && nx != &7 && nx != &9)
                {
                    moves.push((
                        (hallway_x, hallway_y),
                        self.cost * ((y - hallway_y) + (hallway_x - x)),
                    ));
                }

                for hallway_x in (1..x)
                    .rev()
                    .take_while(|nx| state.players.iter().all(|p| p.pos != (*nx, hallway_y)))
                    .filter(|nx| nx != &3 && nx != &5 && nx != &7 && nx != &9)
                {
                    moves.push((
                        (hallway_x, hallway_y),
                        self.cost * ((y - hallway_y) + (x - hallway_x)),
                    ));
                }
            }
        } else {
            let destination_x = self.destination_x();
            if !state.players.iter().any(|p| {
                let (px, _) = p.pos;
                px == destination_x && p.typ != self.typ
            }) {
                if destination_x > x {
                    if let Some(pos) = ((x + 1)..=destination_x)
                        .map(|nx| (nx, y))
                        .chain((y..=3).map(|ny| (destination_x, ny)))
                        .take_while(|pos| state.players.iter().all(|p| &p.pos != pos))
                        .filter(|(_, y)| *y >= 2)
                        .last()
                    {
                        let (nx, ny) = pos;
                        moves.push((pos, self.cost * ((nx - x) + (ny - y))));
                    }
                } else {
                    if let Some(pos) = (destination_x..x)
                        .rev()
                        .map(|nx| (nx, y))
                        .chain((y..=3).map(|ny| (destination_x, ny)))
                        .take_while(|pos| state.players.iter().all(|p| &p.pos != pos))
                        .filter(|(_, y)| *y >= 2)
                        .last()
                    {
                        let (nx, ny) = pos;
                        moves.push((pos, self.cost * ((x - nx) + (ny - y))));
                    }
                }
            }
        }

        moves
    }
}

#[derive(Eq, PartialEq)]
struct World {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    players: Vec<Player>,
    len: usize,
    est: usize,
}

impl State {
    fn finished(&self) -> bool {
        self.players.iter().all(|p| p.finished())
    }

    fn duplication_key(&self) -> u128 {
        let mut keys: Vec<(u32, u128)> = self
            .players
            .iter()
            .map(|p| {
                let (x, y) = p.pos;
                (p.typ, ((x << 2) | y) as u128)
            })
            .collect();
        keys.sort();
        keys.into_iter().fold(0, |acc, (_, key)| (acc << 6) | key)
    }

    fn estimate(&self) -> usize {
        self.len
            + self
                .players
                .iter()
                .map(|p| {
                    let dest_x = p.destination_x();
                    let (x, y) = p.pos;
                    if x == dest_x {
                        0
                    } else {
                        p.cost
                            * (std::cmp::max(x, dest_x) - std::cmp::min(x, dest_x)
                                + (if y >= 2 { (y - 1) + 1 } else { 1 }))
                    }
                })
                .sum::<usize>()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.est, self.len).cmp(&(other.est, other.len))
        // (self.estimate(), self.len).cmp(&(other.estimate(), other.len))
        // self.len.cmp(&other.len)
        // (self.len, self.est).cmp(&(other.len, other.est))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn print_state(world: &World, state: &State) {
    println!(
        "{}",
        world
            .tiles
            .iter()
            .enumerate()
            .map(|(r, row)| row
                .iter()
                .enumerate()
                .map(
                    |(c, tile)| if let Some(p) = state.players.iter().find(|p| p.pos == (c, r)) {
                        char::try_from(p.typ + u32::from('A')).unwrap().to_string()
                    } else {
                        match tile {
                            Wall => '#'.to_string(),
                            Floor => '.'.to_string(),
                            Destination(_) => '.'.to_string(),
                        }
                    }
                )
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn parse_world(lines: &[String]) -> (World, Vec<Player>) {
    let mut players = Vec::new();
    let tiles: Vec<Vec<Tile>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' | ' ' => Wall,
                    '.' => Floor,
                    a => {
                        let typ = u32::from(a) - u32::from('A');
                        players.push(Player {
                            typ,
                            cost: 10usize.pow(typ),
                            pos: (x, y),
                            moves: 0,
                        });
                        Destination((x as u32 - 3) / 2)
                    }
                })
                .collect()
        })
        .collect();

    (World { tiles }, players)
}

fn dijkstra<'world>(world: &'world World, players: Vec<Player>) -> Option<State> {
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut shortest: HashMap<u128, usize> = HashMap::new();

    queue.push(Reverse(State {
        players,
        len: 0,
        est: 0,
    }));

    while let Some(Reverse(state)) = queue.pop() {
        // println!("{} {}", queue.len(), state.len);
        // println!("{} {} {}", queue.len(), state.len, state.estimate());
        // print_state(world, &state);
        // println!();

        if state.finished() {
            return Some(state);
        } else {
            let short = shortest.entry(state.duplication_key()).or_insert(state.len);

            if state.len <= *short {
                *short = state.len;

                for (player_i, player) in state
                    .players
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.can_move())
                {
                    for (pos, cost) in player.available_moves(&state) {
                        let mut new_state = state.clone();
                        new_state.players[player_i].pos = pos;
                        new_state.players[player_i].moves += 1;
                        new_state.len += cost;
                        new_state.est = new_state.estimate();

                        let short2 = shortest
                            .entry(new_state.duplication_key())
                            .or_insert(new_state.len + 1);

                        if new_state.len < *short2 {
                            *short2 = new_state.len;
                            queue.push(Reverse(new_state));
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn solve(lines: &[String]) -> Solution {
    let (world, players) = parse_world(lines);

    let a_solution = dijkstra(&world, players).unwrap().len;
    let b_solution = 0;
    (a_solution.to_string(), b_solution.to_string())
}
