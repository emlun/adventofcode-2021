use crate::common::Solution;

#[derive(Clone, Debug)]
enum SnailNumber {
    Simple(u32),
    Pair(Box<(SnailNumber, SnailNumber)>),
}
use SnailNumber::Pair;
use SnailNumber::Simple;

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Simple(simple) => write!(f, "{}", simple),
            Pair(leftright) => {
                let (left, right) = leftright.as_ref();
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
        Pair(Box::new((left, right)))
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

    fn reduce(&mut self) {
        if self.explode() || self.split() {
            self.reduce();
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
                Pair(leftright) => {
                    let (left, right) = leftright.as_mut();
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
            let mut exploded = Simple(0);
            std::mem::swap(exploder, &mut exploded);
            if let Pair(lexrex) = exploded {
                if let (Simple(left_exploded), Simple(right_exploded)) = *lexrex {
                    if let Some(left_recipient) = left_recipient {
                        *left_recipient += left_exploded;
                    }
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

    fn split(&mut self) -> bool {
        match self {
            Simple(simple) => {
                if *simple >= 10 {
                    *self = Self::pair(Simple(*simple / 2), Simple(*simple / 2 + *simple % 2));
                    true
                } else {
                    false
                }
            }
            Pair(leftright) => {
                let (left, right) = leftright.as_mut();
                left.split() || right.split()
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Simple(simple) => *simple,
            Pair(leftright) => {
                let (left, right) = leftright.as_ref();
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }
}

impl std::ops::Add for SnailNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> <Self as std::ops::Add>::Output {
        let mut result = Self::pair(self, rhs);
        result.reduce();
        result
    }
}

impl std::ops::Add for &SnailNumber {
    type Output = SnailNumber;
    fn add(self, rhs: Self) -> <Self as std::ops::Add>::Output {
        self.clone() + rhs.clone()
    }
}

impl std::ops::AddAssign for SnailNumber {
    fn add_assign(&mut self, rhs: Self) {
        let mut tmp = Simple(0);
        std::mem::swap(self, &mut tmp);
        tmp = Self::pair(tmp, rhs);
        tmp.reduce();
        std::mem::swap(self, &mut tmp);
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<SnailNumber> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    let sol_b = (0..nums.len())
        .flat_map(|i| (0..nums.len()).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| (&nums[i] + &nums[j]).magnitude())
        .max()
        .unwrap();

    let sol_a = nums
        .into_iter()
        .reduce(std::ops::Add::add)
        .unwrap()
        .magnitude();

    (sol_a.to_string(), sol_b.to_string())
}
