use crate::common::Solution;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Mod(usize, Operand),
    Eql(usize, Operand),
}
use Instruction::*;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Literal(i64),
    Register(usize),
}
use Operand::*;

impl FromStr for Instruction {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut splits = s.split_whitespace();
        let ins = splits.next().unwrap();
        if let Register(op1) = splits.next().unwrap().parse::<Operand>()? {
            match ins {
                "inp" => Ok(Inp(op1)),
                other => {
                    let op2 = splits.next().unwrap().parse()?;
                    match other {
                        "add" => Ok(Add(op1, op2)),
                        "mul" => Ok(Mul(op1, op2)),
                        "div" => Ok(Div(op1, op2)),
                        "mod" => Ok(Mod(op1, op2)),
                        "eql" => Ok(Eql(op1, op2)),
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
}

impl FromStr for Operand {
    type Err = <i64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "w" | "x" | "y" | "z" => Ok(Register(
                (u32::from(s.chars().next().unwrap()) - u32::from('w')) as usize,
            )),
            other => Ok(Literal(other.parse()?)),
        }
    }
}

fn run(program: &[Instruction], input: &[i64]) -> [i64; 4] {
    let mut registers = [0, 0, 0, 0];
    let mut input = input.iter();

    fn read(regs: &mut [i64; 4], op: &Operand) -> i64 {
        match op {
            Literal(lit) => *lit,
            Register(addr) => regs[*addr],
        }
    }

    for instruction in program {
        match instruction {
            Inp(addr) => {
                registers[*addr] = *input.next().unwrap();
            }
            Add(addr, op2) => {
                registers[*addr] = registers[*addr] + read(&mut registers, op2);
            }
            Mul(addr, op2) => {
                registers[*addr] = registers[*addr] * read(&mut registers, op2);
            }
            Div(addr, op2) => {
                registers[*addr] = registers[*addr] / read(&mut registers, op2);
            }
            Mod(addr, op2) => {
                registers[*addr] = registers[*addr] % read(&mut registers, op2);
            }
            Eql(addr, op2) => {
                registers[*addr] = (registers[*addr] == read(&mut registers, op2)).into();
            }
        }
    }

    registers
}

fn step(mut z: i64, w: i64, d: i64, a: i64, b: i64) -> i64 {
    let x = z % 26 + a;
    z = z / d;
    dbg!(w, x);
    if x == w {
        z
    } else {
        z * 26 + w + b
    }
}

fn run_hardcode<I: Iterator<Item = i64>>(mut input: I) -> i64 {
    let mut z = 0;

    dbg!("WOOHOO!");

    z = step(z, input.next().unwrap(), 1, 11, 8);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 12, 8);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 10, 12);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -8, 10);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 15, 2);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 15, 8);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -11, 4);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 10, 9);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -3, 10);
    dbg!(z);

    z = step(z, input.next().unwrap(), 1, 15, 3);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -3, 7);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -1, 7);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -10, 2);
    dbg!(z);

    z = step(z, input.next().unwrap(), 26, -16, 2);
    dbg!(z);

    z
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<Instruction> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    // dbg!(&program);
    // let sol_a = (0..99_999_999_999_999_i64)
    let sol_a = (0..=99598963999971_i64)
        .rev()
        .filter(|i| !i.to_string().contains('0'))
        .find(|i| {
            if i % 1000000 == 111111 {
                println!("{}", i);
            }
            let z = run_hardcode(
                i.to_string()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap()),
            );
            z == 0
        })
        .unwrap();

    let sol_b = [93151411711211_i64]
        .into_iter()
        .filter(|i| !i.to_string().contains('0'))
        .find(|i| {
            if i % 1000000 == 111111 {
                println!("{}", i);
            }
            let z = run_hardcode(
                i.to_string()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap()),
            );
            z == 0
        })
        .unwrap();

    (sol_a.to_string(), sol_b.to_string())
}
