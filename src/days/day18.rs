use crate::common::Solution;

#[derive(Debug)]
enum SnailNumber {
    Simple(u32),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}
use SnailNumber::Pair;
use SnailNumber::Simple;

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Simple(simple) => write!(f, "{}", simple),
            Pair(left, right) => {
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}

impl std::str::FromStr for SnailNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        Self::parse(&mut s.chars()).ok_or(())
    }
}

impl SnailNumber {
    fn pair(left: Self, right: Self) -> Self {
        Pair(Box::new(left), Box::new(right))
    }

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Option<SnailNumber> {
        input.next().map(|first| match first {
            '[' => {
                let a = Self::parse(input).unwrap();
                let comma = input.next().unwrap();
                let b = Self::parse(input).unwrap();
                let result = Self::pair(a, b);
                let end = input.next().unwrap();
                assert_eq!(comma, ',');
                assert_eq!(end, ']');
                result
            }
            _ => Simple(first.to_digit(10).unwrap()),
        })
    }

    fn reduce(mut self) -> Self {
        if self.explode() {
            // println!("Exploded: {}", self);
            self.reduce()
        } else {
            match self.split() {
                Ok(modified) => {
                    // println!("Splitted: {}", modified);
                    modified.reduce()
                }
                Err(unmodified) => {
                    // println!("Unmodified: {}", unmodified);
                    unmodified
                }
            }
        }
    }

    fn find_explosion<'tree>(
        &'tree mut self,
        level: usize,
        left_recipient: &mut Option<&'tree mut u32>,
        exploder: &mut Option<&'tree mut Self>,
        right_recipient: &mut Option<&'tree mut u32>,
    ) {
        if right_recipient.is_none() {
            match self {
                Simple(simple) => {
                    if exploder.is_none() {
                        *left_recipient = Some(simple);
                    } else {
                        *right_recipient = Some(simple);
                    }
                }
                pair @ Pair(..) if level >= 4 && exploder.is_none() => {
                    *exploder = Some(pair);
                }
                Pair(left, right) => {
                    left.find_explosion(level + 1, left_recipient, exploder, right_recipient);
                    right.find_explosion(level + 1, left_recipient, exploder, right_recipient);
                }
            }
        }
    }

    fn explode(&mut self) -> bool {
        let mut left_recipient: Option<&mut u32> = None;
        let mut exploder: Option<&mut Self> = None;
        let mut right_recipient: Option<&mut u32> = None;
        self.find_explosion(0, &mut left_recipient, &mut exploder, &mut right_recipient);
        if let Some(exploder) = exploder {
            // println!(
            //     "Exploding: {:?} <- {} -> {:?}",
            //     left_recipient, exploder, right_recipient
            // );

            let mut exploded = Simple(0);
            std::mem::swap(exploder, &mut exploded);
            if let Pair(lex, rex) = exploded {
                if let Simple(left_exploded) = *lex {
                    if let Some(left_recipient) = left_recipient {
                        *left_recipient += left_exploded;
                    }
                } else {
                    unreachable!();
                }

                if let Simple(right_exploded) = *rex {
                    if let Some(right_recipient) = right_recipient {
                        *right_recipient += right_exploded;
                    }
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
            true
        } else {
            false
        }
    }

    fn split(self) -> Result<Self, Self> {
        match self {
            Simple(simple) => {
                if simple >= 10 {
                    Ok(Self::pair(
                        Simple(simple / 2),
                        Simple(simple / 2 + simple % 2),
                    ))
                } else {
                    Err(self)
                }
            }
            Pair(left, right) => match left.split() {
                Ok(left_splitted) => Ok(Self::pair(left_splitted, *right)),
                Err(left_unmodified) => match right.split() {
                    Ok(right_splitted) => Ok(Self::pair(left_unmodified, right_splitted)),
                    Err(right_unmodified) => Err(Self::pair(left_unmodified, right_unmodified)),
                },
            },
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Simple(simple) => *simple,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl std::ops::Add for SnailNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> <Self as std::ops::Add>::Output {
        // println!("self: {}", self);
        // println!("rhs: {}", rhs);
        let result = Self::pair(self, rhs);
        // println!("added: {}", result);
        let reduced = result.reduce();
        // println!("add reduced: {}", reduced);
        reduced
    }
}

impl std::ops::AddAssign for SnailNumber {
    fn add_assign(&mut self, rhs: Self) {
        // println!("self: {}", self);
        // println!("rhs: {}", rhs);
        let mut tmp = Self::pair(Simple(0), rhs);
        std::mem::swap(self, &mut tmp);
        match self {
            Pair(left, _) => {
                std::mem::swap(&mut **left, &mut tmp);
            }
            _ => unreachable!(),
        }
        // println!("add_assigned: {}", self);
        std::mem::swap(self, &mut tmp);
        tmp = tmp.reduce();
        std::mem::swap(self, &mut tmp);
        // println!("add_assigned reduced: {}", self);
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<SnailNumber> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    // for num in &nums {
    // println!("{}", &num);
    // }

    // println!(
    //     "{}",
    //     "[[[[4,3],4],4],[7,[[8,4],9]]]"
    //         .parse::<SnailNumber>()
    //         .unwrap()
    //         + "[1,1]".parse().unwrap()
    // );

    let sol_a = nums
        .into_iter()
        .reduce(std::ops::Add::add)
        .unwrap()
        .magnitude();
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
