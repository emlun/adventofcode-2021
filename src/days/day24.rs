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

fn run_hardcode<I: Iterator<Item = i64>>(mut input: I) -> i64 {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 12;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -8;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 10;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 2;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 8;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 4;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 9;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -3;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 10;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 1;
    x = x + 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 3;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -3;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 7;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -1;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 7;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 2;
    y = y * x;
    z = z + y;
    w = input.next().unwrap();
    x = x * 0;
    x = x + z;
    x = x % 26;
    z = z / 26;
    x = x + -16;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = y * 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = y * 0;
    y = y + w;
    y = y + 2;
    y = y * x;
    z = z + y;

    z
}

pub fn solve(lines: &[String]) -> Solution {
    let program: Vec<Instruction> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    dbg!(&program);

    let sol_a = (0..99_999_999_999_999_i64)
        .rev()
        .filter(|i| !i.to_string().contains('0'))
        .find(|i| {
            println!("{}", i);
            let z = run_hardcode(
                i.to_string()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap()),
            );
            z == 0
        })
        .unwrap();
    let sol_b = 0;

    (sol_a.to_string(), sol_b.to_string())
}
