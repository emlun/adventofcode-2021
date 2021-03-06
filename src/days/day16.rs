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
    bits.by_ref()
        .take(bit_len)
        .fold(0, |acc, bit| (acc << 1) | u64::from(bit))
}

impl Packet {
    fn parse<I: Iterator<Item = u8>>(bits: &mut I) -> Option<Packet> {
        bits.next().map(|first| {
            let ver = (first << 2) | (u8::try_from(read_num(bits, 2)).unwrap());
            let typ = u8::try_from(read_num(bits, 3)).unwrap();
            Packet {
                ver,
                typ,
                body: if typ == 4 {
                    Literal({
                        let mut acc = 0;
                        while let Some(num_continues) = bits.next() {
                            acc = (acc << 4) | read_num(bits, 4);
                            if num_continues == 0 {
                                break;
                            }
                        }
                        acc
                    })
                } else {
                    let len_type = bits.next().unwrap();
                    if len_type == 0 {
                        let bit_len = read_num(bits, 15);
                        let mut subpacket_bits = bits
                            .by_ref()
                            .take(usize::try_from(bit_len).unwrap())
                            .collect::<Vec<u8>>()
                            .into_iter();
                        Subpackets(
                            std::iter::from_fn(|| Packet::parse(&mut subpacket_bits)).collect(),
                        )
                    } else {
                        let subpacket_len = usize::try_from(read_num(bits, 11)).unwrap();
                        Subpackets(
                            std::iter::from_fn(|| Packet::parse(bits))
                                .take(subpacket_len)
                                .collect(),
                        )
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
                _ => {
                    let a = subs[0].compute_value();
                    let b = subs[1].compute_value();
                    (match self.typ {
                        5 => a > b,
                        6 => a < b,
                        7 => a == b,
                        _ => unreachable!(),
                    })
                    .into()
                }
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
