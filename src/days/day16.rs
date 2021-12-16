use crate::common::Solution;

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Subpackets(Vec<Packet>),
}
use PacketBody::Literal;
use PacketBody::Subpackets;

#[derive(Debug)]
struct Packet {
    ver: u8,
    typ: u8,
    body: PacketBody,
}

fn read_num<I: Iterator<Item = u8>>(bits: &mut I, bit_len: usize) -> u64 {
    (0..bit_len).fold(0, |acc, _| (acc << 1) | u64::from(bits.next().unwrap()))
}

impl Packet {
    fn parse<I: Iterator<Item = u8>>(bits: &mut I) -> Option<Packet> {
        bits.next().map(|first| {
            let ver = (first << 2) | (read_num(bits, 2) as u8);
            let typ = read_num(bits, 3) as u8;
            Packet {
                ver,
                typ,
                body: match typ {
                    4 => {
                        let mut body_nibbles = Vec::new();
                        while let Some(first_bit) = bits.next() {
                            body_nibbles.push(read_num(bits, 4));
                            if first_bit == 0 {
                                break;
                            }
                        }
                        Literal(
                            body_nibbles
                                .into_iter()
                                .fold(0, |acc, nibble| (acc << 4) | u64::from(nibble)),
                        )
                    }
                    _ => {
                        let len_type = bits.next().unwrap();
                        if len_type == 0 {
                            let bit_len = read_num(bits, 15);
                            let subpacket_bits: Vec<u8> =
                                (0..bit_len).flat_map(|_| bits.next()).collect();
                            let mut subpacket_bits = subpacket_bits.into_iter();
                            let mut subpackets: Vec<Packet> = Vec::new();
                            while let Some(subpacket) = Packet::parse(&mut subpacket_bits) {
                                subpackets.push(subpacket);
                            }
                            Subpackets(subpackets)
                        } else {
                            let subpacket_len = usize::try_from(read_num(bits, 11)).unwrap();
                            let mut subpackets: Vec<Packet> = Vec::new();
                            while let Some(subpacket) = Packet::parse(bits) {
                                subpackets.push(subpacket);
                                if subpackets.len() == subpacket_len {
                                    break;
                                }
                            }
                            Subpackets(subpackets)
                        }
                    }
                },
            }
        })
    }

    fn sum_versions(&self) -> u64 {
        u64::from(self.ver)
            + match &self.body {
                Literal(_) => 0,
                Subpackets(subs) => subs.iter().map(|s| s.sum_versions()).sum(),
            }
    }

    fn compute_value(&self) -> u64 {
        match &self.body {
            Literal(value) => *value,
            Subpackets(subs) => match self.typ {
                0 => subs.iter().map(|s| s.compute_value()).sum(),
                1 => subs.iter().map(|s| s.compute_value()).product(),
                2 => subs.iter().map(|s| s.compute_value()).min().unwrap(),
                3 => subs.iter().map(|s| s.compute_value()).max().unwrap(),
                5 => {
                    let a = subs[0].compute_value();
                    let b = subs[1].compute_value();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = subs[0].compute_value();
                    let b = subs[1].compute_value();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = subs[0].compute_value();
                    let b = subs[1].compute_value();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let mut bits = lines
        .iter()
        .filter(|l| !l.is_empty())
        .flat_map(|l| (0..(l.len())).map(|i| u8::from_str_radix(&l[i..=i], 16).unwrap()))
        .flat_map(|b| (0..4).rev().map(move |i| (b >> i) & 0x01));

    let message: Packet = Packet::parse(&mut bits).unwrap();

    let sol_a = message.sum_versions();
    let sol_b = message.compute_value();

    (sol_a.to_string(), sol_b.to_string())
}
