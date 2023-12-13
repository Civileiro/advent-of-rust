use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{char, u64},
    combinator::value,
    multi::separated_list1,
    sequence::preceded,
    IResult, InputTakeAtPosition,
};
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
enum Status {
    Operational = b'.' as isize,
    Damaged = b'#' as isize,
    Unknown = b'?' as isize,
}

impl Status {
    fn is_damaged(&self) -> bool {
        self == &Status::Damaged
    }
    fn is_operational(&self) -> bool {
        self == &Status::Operational
    }
}

impl Status {
    #[allow(dead_code)]
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Status::Operational, char('.')),
            value(Status::Damaged, char('#')),
            value(Status::Unknown, char('?')),
        ))(input)
    }

    #[allow(dead_code)]
    fn transmute_from_bytes(bytes: &[u8]) -> Option<&[Self]> {
        if !bytes.iter().all(|&c| Self::is_status_byte(c)) {
            return None;
        }
        // SAFETY: we checked that every byte is valid
        let res = unsafe { Self::transmute_from_bytes_unchecked(bytes) };
        Some(res)
    }

    unsafe fn transmute_from_bytes_unchecked(bytes: &[u8]) -> &[Self] {
        unsafe { std::mem::transmute(bytes) }
    }

    fn is_status_byte(c: u8) -> bool {
        Self::iter().any(|s| s as u8 == c)
    }

    fn parse_list(input: &str) -> IResult<&str, &[Status]> {
        let (i, status_chars) =
            input.split_at_position_complete(|c| !Status::is_status_byte(c as u8))?;
        // SAFETY: we only took bytes which are valid statuses
        let statuses = unsafe { Status::transmute_from_bytes_unchecked(status_chars.as_bytes()) };
        Ok((i, statuses))
    }
}

fn parse_line(input: &str) -> IResult<&str, (&[Status], Vec<usize>)> {
    let (i, statuses) = preceded(sp, Status::parse_list)(input)?;
    let (i, sizes) = preceded(sp, separated_list1(char(','), u64))(i)?;
    let sizes = sizes.into_iter().map(|n| n.try_into().unwrap()).collect();
    Ok((i, (statuses, sizes)))
}

pub type NumArrangements = u64;

fn status_arrangements(
    cache: &mut HashMap<(usize, usize), NumArrangements>,
    statuses: &[Status],
    sizes: &[usize],
) -> NumArrangements {
    let key = (statuses.len(), sizes.len());
    if let Some(&res) = cache.get(&key) {
        return res;
    }

    let Some((size, rest_sizes)) = sizes.split_first() else {
        // there are no sizes left
        // if theres no damage left to be accounted for then 1 else 0
        let res = if statuses.iter().all(|s| !s.is_damaged()) {
            1
        } else {
            0
        };
        cache.insert(key, res);
        return res;
    };

    let rem_size: usize = rest_sizes.iter().sum();
    if statuses.len() < size + rem_size {
        let res = 0;
        cache.insert(key, res);
        return res;
    }

    let mut res = 0;

    let max_start = statuses.len() - size - rem_size;
    for start in 0..=max_start {
        let can_be_separated =
            start + size == statuses.len() || !statuses[start + size].is_damaged();
        let possibly_all_damaged = statuses[start..start + size]
            .iter()
            .all(|s| !s.is_operational());

        if can_be_separated && possibly_all_damaged {
            let next_slice_start = (start + size + 1).min(statuses.len());
            res += status_arrangements(cache, &statuses[next_slice_start..], rest_sizes)
        }

        let has_left_damaged = statuses[start].is_damaged();
        if has_left_damaged {
            break;
        }
    }
    cache.insert(key, res);
    res
}

pub fn day12_1(input: &str) -> Result<u64, ParseError> {
    let mut res = 0;

    let mut cache = HashMap::new();
    for line in input.lines() {
        let (_, (statuses, sizes)) = parse_line(line)?;

        res += status_arrangements(&mut cache, statuses, &sizes);

        cache.clear();
    }
    Ok(res)
}

pub fn day12_2(input: &str) -> Result<u64, ParseError> {
    let mut res = 0;

    let mut cache = HashMap::new();
    let mut unfolded_statuses = Vec::new();
    let mut unfolded_sizes = Vec::new();
    for line in input.lines() {
        let (_, (statuses, sizes)) = parse_line(line)?;
        unfolded_statuses.extend(statuses.iter().copied());
        unfolded_sizes.extend(sizes.iter().copied());
        for _ in 0..4 {
            unfolded_statuses.push(Status::Unknown);
            unfolded_statuses.extend(statuses.iter().copied());
            unfolded_sizes.extend(sizes.iter().copied());
        }

        res += status_arrangements(&mut cache, &unfolded_statuses, &unfolded_sizes);

        cache.clear();
        unfolded_statuses.clear();
        unfolded_sizes.clear();
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{day12_1, day12_2};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_day12_1() {
        let res = day12_1(INPUT);
        assert_eq!(res, Ok(21))
    }

    #[test]
    fn test_day12_2() {
        let res = day12_2(INPUT);

        assert_eq!(res, Ok(525152))
    }
}
