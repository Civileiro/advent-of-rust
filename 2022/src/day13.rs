#![allow(dead_code)]

use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePacketError;

struct SplitPackets<'a> {
    s: &'a str,
}

impl<'a> Iterator for SplitPackets<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }
        let mut curr = 0;
        let mut balance = 0;
        for c in self.s.chars() {
            curr += c.len_utf8();
            if c == '[' {
                balance += 1;
                continue;
            }
            if c == ']' {
                balance -= 1;
                continue;
            }
            if balance != 0 {
                continue;
            }
            if c == ',' {
                let match_ = unsafe { self.s.get_unchecked(0..curr - 1) };
                self.s = unsafe { self.s.get_unchecked(curr..) };
                return Some(match_);
            }
        }
        let res = core::mem::take(&mut self.s);
        Some(res)
    }
}

impl std::str::FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with('[') && s.ends_with(']') {
            let stripped = s
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .ok_or(ParsePacketError)?;
            let packet_list = Self::split_packets(stripped)
                .map(Self::from_str)
                .collect::<Result<_, _>>()?;
            Ok(Packet::List(packet_list))
        } else {
            let int = s.parse::<i32>().map_err(|_| ParsePacketError)?;
            Ok(Packet::Int(int))
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(n1), Packet::Int(n2)) => n1.cmp(n2),
            (n1 @ Packet::Int(_), l2 @ Packet::List(_)) => Packet::list_wrap(n1.clone()).cmp(l2),
            (l1 @ Packet::List(_), n2 @ Packet::Int(_)) => l1.cmp(&Packet::list_wrap(n2.clone())),
            (Packet::List(v1), Packet::List(v2)) => {
                for p in v1.iter().zip_longest(v2) {
                    use itertools::EitherOrBoth::*;
                    match p {
                        Left(_) => return std::cmp::Ordering::Greater,
                        Right(_) => return std::cmp::Ordering::Less,
                        Both(l1, l2) => {
                            let comp = l1.cmp(l2);
                            if comp != std::cmp::Ordering::Equal {
                                return comp;
                            }
                        }
                    }
                }
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl Packet {
    fn split_packets(s: &str) -> SplitPackets {
        SplitPackets { s }
    }
    fn list_wrap(n: Packet) -> Packet {
        Packet::List(vec![n])
    }
    // pub fn in_packet_order(&self, other: &Self) -> std::cmp::Ordering {

    // }
}

pub fn day13_1(input: &str) -> usize {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .tuples()
        .map(|(s1, s2)| {
            let p1 = Packet::from_str(s1).unwrap();
            let p2 = Packet::from_str(s2).unwrap();
            p1 < p2
        })
        .enumerate()
        .filter(|(_, b)| *b)
        .fold(0, |acc, (i, _)| acc + i + 1)
}

pub fn day13_2(input: &str) -> usize {
    let div1 = Packet::from_str("[[2]]").unwrap();
    let div2 = Packet::from_str("[[6]]").unwrap();
    let mut packets: Vec<_> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| Packet::from_str(line).unwrap())
        .chain([div1.clone(), div2.clone()])
        .collect();
    packets.sort_unstable();
    let pos1 = packets.iter().position(|p| p == &div1).unwrap() + 1;
    let pos2 = packets.iter().skip(pos1).position(|p| p == &div2).unwrap() + 1;

    pos1 * pos2
}

const _TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
