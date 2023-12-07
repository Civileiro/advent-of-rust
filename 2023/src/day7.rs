use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, u32},
    combinator::value,
    multi::fill,
    sequence::preceded,
    IResult,
};

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Card {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    T = 8,
    J = 9,
    Q = 10,
    K = 11,
    A = 12,
}

const NUM_CARDS: usize = 13;

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Two, char('2')),
            value(Self::Three, char('3')),
            value(Self::Four, char('4')),
            value(Self::Five, char('5')),
            value(Self::Six, char('6')),
            value(Self::Seven, char('7')),
            value(Self::Eight, char('8')),
            value(Self::Nine, char('9')),
            value(Self::T, char('T')),
            value(Self::J, char('J')),
            value(Self::Q, char('Q')),
            value(Self::K, char('K')),
            value(Self::A, char('A')),
        ))(input)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn from_hand(hand: [Card; HAND_SIZE], j_as_joker: bool) -> Self {
        let mut counts = [0; NUM_CARDS];
        for card in hand {
            counts[card as usize] += 1;
        }
        let joker_count = if j_as_joker {
            let n = counts[Card::J as usize];
            counts[Card::J as usize] = 0;
            n
        } else {
            0
        };
        let mut first_highest = 0;
        let mut second_highest = 0;
        for count in counts {
            if count > first_highest {
                second_highest = first_highest;
                first_highest = count;
            } else if count > second_highest {
                second_highest = count;
            }
        }
        match (joker_count + first_highest, second_highest) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, _) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, _) => Self::OnePair,
            (1, _) => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

const HAND_SIZE: usize = 5;

struct Hand {
    power: u32,
    bid: u32,
}

impl Hand {
    fn parse<P>(power_func: P) -> impl Fn(&str) -> IResult<&str, Self>
    where
        P: Fn([Card; HAND_SIZE]) -> u32,
    {
        move |input| {
            let mut hand = [Card::Two; HAND_SIZE];
            let (i, ()) = preceded(sp, fill(Card::parse, &mut hand))(input)?;
            let (i, bid) = preceded(sp, u32)(i)?;
            let power = power_func(hand);
            Ok((i, Self { power, bid }))
        }
    }

    fn power(hand: [Card; HAND_SIZE]) -> u32 {
        let hand_type = HandType::from_hand(hand, false);
        let mut power = 0;
        power |= (hand_type as u32) << 28;
        for (i, &card) in hand.iter().enumerate() {
            let card_power = card as u32;
            power |= card_power << (4 * (HAND_SIZE - 1 - i));
        }
        power
    }

    fn power_joker(hand: [Card; HAND_SIZE]) -> u32 {
        let hand_type = HandType::from_hand(hand, true);
        let mut power = 0;
        power |= (hand_type as u32) << 28;
        for (i, &card) in hand.iter().enumerate() {
            let card_power = if card != Card::J { card as u32 + 1 } else { 0 };
            power |= card_power << (4 * (HAND_SIZE - 1 - i));
        }
        power
    }
}

fn process_day<P>(input: &str, line_parser: P) -> Result<u32, ParseError>
where
    P: Fn(&str) -> IResult<&str, Hand>,
{
    let mut hands: Vec<_> = input
        .lines()
        .map(line_parser)
        .map_ok(|(_, hand)| hand)
        .collect::<Result<Vec<_>, _>>()?;
    hands.sort_unstable_by_key(|hand| hand.power);
    let total = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();
    Ok(total)
}

pub fn day7_1(input: &str) -> Result<u32, ParseError> {
    let line_parser = Hand::parse(Hand::power);
    process_day(input, line_parser)
}

pub fn day7_2(input: &str) -> Result<u32, ParseError> {
    let line_parser = Hand::parse(Hand::power_joker);
    process_day(input, line_parser)
}

#[cfg(test)]
mod tests {
    use super::{day7_1, day7_2};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_day7_1() {
        let res = day7_1(INPUT);
        assert_eq!(res, Ok(6440))
    }

    #[test]
    fn test_day7_2() {
        let res = day7_2(INPUT);
        assert_eq!(res, Ok(5905))
    }
}
