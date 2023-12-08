use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, satisfy},
    combinator::value,
    multi::{fill, fold_many0, many0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use num::Integer;

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(Self::Left, char('L')), value(Self::Right, char('R'))))(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct NodeId(u16);

impl NodeId {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut id_chars = [' '; 3];
        let (i, ()) = fill(satisfy(|c| c.is_ascii_uppercase()), &mut id_chars)(input)?;
        let node_id = Self::from_chars(&id_chars);
        Ok((i, node_id))
    }

    fn ends_with(&self, ascii_letter: u8) -> bool {
        debug_assert!(ascii_letter.is_ascii_uppercase());
        (self.0 >> 10) == (ascii_letter - b'A').into()
    }

    fn from_chars(chars: &[char; 3]) -> Self {
        let mut node_id = 0;
        for (index, &c) in chars.iter().enumerate() {
            let c = c as u8 - b'A';
            node_id |= (c as u16) << (5 * index);
        }
        Self(node_id)
    }
}

#[derive(Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    pub fn parse(input: &str) -> IResult<&str, (NodeId, Self)> {
        let (i, id) = preceded(sp, NodeId::parse)(input)?;
        let (i, _) = tag(" = ")(i)?;
        let (i, (left, right)) = delimited(
            char('('),
            separated_pair(NodeId::parse, tag(", "), NodeId::parse),
            char(')'),
        )(i)?;
        Ok((i, (id, Self { left, right })))
    }
}

fn parse_input(input: &str) -> Result<(Vec<Direction>, HashMap<NodeId, Node>), ParseError> {
    let (i, dirs) = many0(Direction::parse)(input)?;
    let (_, nodes) = fold_many0(Node::parse, HashMap::new, |mut acc, (id, node)| {
        acc.insert(id, node);
        acc
    })(i)?;
    Ok((dirs, nodes))
}

pub fn day8_1(input: &str) -> Result<u64, ParseError> {
    let (dirs, nodes) = parse_input(input)?;
    let mut curr_node = NodeId::from_chars(&['A', 'A', 'A']);
    let target_node = NodeId::from_chars(&['Z', 'Z', 'Z']);
    let mut steps = 0;
    for dir in dirs.into_iter().cycle() {
        steps += 1;
        let Node { left, right } = nodes[&curr_node];
        curr_node = match dir {
            Direction::Left => left,
            Direction::Right => right,
        };
        if curr_node == target_node {
            break;
        }
    }
    Ok(steps)
}

pub fn day8_2(input: &str) -> Result<u64, ParseError> {
    let (dirs, nodes) = parse_input(input)?;
    let starting_nodes: HashSet<NodeId> =
        HashSet::from_iter(nodes.keys().copied().filter(|node| node.ends_with(b'A')));
    let mut cycle_lengths = Vec::with_capacity(starting_nodes.len());
    for start_node in &starting_nodes {
        let mut curr_node = *start_node;
        let mut steps = 0;
        for &dir in dirs.iter().cycle() {
            steps += 1;
            let Node { left, right } = nodes[&curr_node];
            curr_node = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };
            if curr_node.ends_with(b'Z') {
                cycle_lengths.push(steps);
                break;
            }
        }
    }
    let lcm = cycle_lengths.into_iter().fold(1, |acc, n| acc.lcm(&n));
    Ok(lcm)
}

#[cfg(test)]
mod tests {
    use super::{day8_1, day8_2};

    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    const INPUT3: &str = "LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)
";

    #[test]
    fn test_day8_1() {
        let res = day8_1(INPUT);
        assert_eq!(res, Ok(2))
    }

    #[test]
    fn test_day8_1_2() {
        let res = day8_1(INPUT2);
        assert_eq!(res, Ok(6))
    }

    #[test]
    fn test_day8_2() {
        let res = day8_2(INPUT3);
        assert_eq!(res, Ok(6))
    }
}
