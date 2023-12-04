use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    multi::fold_many0,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use super::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

struct Card {
    amount: u32,
    valid_for: u32,
}

impl Card {
    pub fn new(amount: u32, valid_for: u32) -> Self {
        Self { amount, valid_for }
    }

    pub fn parse_calc_matches(input: &str) -> IResult<&str, u32> {
        let (i, _) = tag("Card")(input)?;
        let (i, _id) = preceded(sp, u32)(i)?;
        let (i, _) = char(':')(i)?;
        let (i, winning) = separated_list1(char(' '), preceded(sp, u32))(i)?;
        let (i, _) = preceded(sp, char('|'))(i)?;
        let (i, matches) = fold_many0(
            preceded(sp, u32),
            || 0,
            |acc, n| {
                if winning.contains(&n) {
                    acc + 1
                } else {
                    acc
                }
            },
        )(i)?;
        Ok((i, matches))
    }

    pub fn matches_to_points(matches: u32) -> u32 {
        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }
}

pub fn day4_1(input: &str) -> Result<u32, ParseError> {
    input
        .lines()
        .map(Card::parse_calc_matches)
        .fold_ok(0, |acc, (_, n)| acc + Card::matches_to_points(n))
}

pub fn day4_2(input: &str) -> Result<u32, ParseError> {
    let mut cards = Vec::<Card>::new();
    let mut total = 0;
    for match_parse in input.lines().map(Card::parse_calc_matches) {
        let (_, matches) = match_parse?;

        let amount = 1 + cards.iter().map(|card| card.amount).sum::<u32>();
        total += amount;

        for card in &mut cards {
            card.valid_for -= 1;
        }
        while let Some(card) = cards.last() {
            if card.valid_for != 0 {
                break;
            }
            cards.pop();
        }
        if matches != 0 {
            let new_card = Card::new(amount, matches);
            let pos = cards.partition_point(|card| card.valid_for > new_card.valid_for);
            cards.insert(pos, new_card);
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_day4_1() {
        let res = day4_1(INPUT);
        assert_eq!(res, Ok(13))
    }

    #[test]
    fn test_day4_2() {
        let res = day4_2(INPUT);
        assert_eq!(res, Ok(30))
    }
}
