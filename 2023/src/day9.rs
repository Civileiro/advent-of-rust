use itertools::Itertools;
use nom::{
    character::complete::{char, i64},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

type SequenceValue = i64;
type Sequence = Vec<SequenceValue>;

fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    preceded(sp, separated_list0(char(' '), i64))(input)
}

fn get_diffs(seq: Sequence) -> Vec<Sequence> {
    let mut diffs = vec![seq];
    loop {
        let prev = diffs.last().unwrap();
        let next: Sequence = prev
            .iter()
            .copied()
            .tuple_windows()
            .map(|(n1, n2)| n2 - n1)
            .collect();
        if !next.iter().any(|n| n != &0) {
            break;
        }
        diffs.push(next);
    }
    diffs
}

fn extrapolate_last(seq: Sequence) -> SequenceValue {
    get_diffs(seq).iter().filter_map(|seq| seq.last()).sum()
}

fn extrapolate_first(seq: Sequence) -> SequenceValue {
    get_diffs(seq)
        .iter()
        .filter_map(|seq| seq.first())
        .fold((0, true), |(acc, add), &n| {
            (acc + if add { n } else { -n }, !add)
        })
        .0
}

fn process_day<F>(input: &str, extrapolate: F) -> Result<SequenceValue, ParseError>
where
    F: Fn(Sequence) -> SequenceValue,
{
    input
        .lines()
        .map(parse_sequence)
        .map_ok(|(_, seq)| extrapolate(seq))
        .fold_ok(0, |acc, n| acc + n)
}

pub fn day9_1(input: &str) -> Result<SequenceValue, ParseError> {
    process_day(input, extrapolate_last)
}

pub fn day9_2(input: &str) -> Result<SequenceValue, ParseError> {
    process_day(input, extrapolate_first)
}

#[cfg(test)]
mod tests {
    use super::{day9_1, day9_2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_day9_1() {
        let res = day9_1(INPUT);
        assert_eq!(res, Ok(114))
    }

    #[test]
    fn test_day9_2() {
        let res = day9_2(INPUT);
        assert_eq!(res, Ok(2))
    }
}
