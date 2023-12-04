use std::{error::Error, fmt::Display};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum Day1Error {
    CouldNotFindDigitOrSpelling,
}

impl Display for Day1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Day1Error {}

fn first_digit(mut s: impl Iterator<Item = u8>) -> Option<u8> {
    let res = s.find(|c| c.is_ascii_digit())? - b'0';
    Some(res)
}

pub fn day1_1(input: &str) -> Result<u32, Day1Error> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let first = first_digit(line.iter().copied())? as u32;
            let last = first_digit(line.iter().copied().rev())? as u32;
            Some(first * 10 + last)
        })
        .map(|opt| opt.ok_or(Day1Error::CouldNotFindDigitOrSpelling))
        .fold_ok(0, |acc, n| acc + n)
}

const DIGITS_SPELL: [&[u8]; 10] = [
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn find_first_digit_with_spelling(
    s: &[u8],
    range: impl Iterator<Item = usize>,
) -> Result<u32, Day1Error> {
    for i in range {
        if s[i].is_ascii_digit() {
            return Ok((s[i] - b'0').into());
        }
        for (digit, spelling) in DIGITS_SPELL.iter().enumerate() {
            if s[i..].starts_with(spelling) {
                return Ok(digit as u32);
            }
        }
    }
    Err(Day1Error::CouldNotFindDigitOrSpelling)
}

pub fn day1_2(input: &str) -> Result<u32, Day1Error> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let first = find_first_digit_with_spelling(line, 0..line.len())?;
            let last = find_first_digit_with_spelling(line, (0..line.len()).rev())?;
            Ok(first * 10 + last)
        })
        .fold_ok(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        let res = day1_1(input);
        assert_eq!(res, Ok(142));
    }

    #[test]
    fn test_day1_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let res = day1_2(input);
        assert_eq!(res, Ok(281));
    }
}
