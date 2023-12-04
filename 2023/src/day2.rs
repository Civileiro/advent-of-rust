use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u32},
    combinator::map,
    multi::separated_list1,
    IResult,
};

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Default)]
struct BallSet {
    red_cnt: u32,
    green_cnt: u32,
    blue_cnt: u32,
}

enum BallColor {
    Red,
    Green,
    Blue,
}

impl BallSet {
    fn parse_ball_group(input: &str) -> IResult<&str, (u32, BallColor)> {
        let (i, _) = char(' ')(input)?;
        let (i, n) = u32(i)?;
        let (i, _) = char(' ')(i)?;
        let (i, color) = alt((
            map(tag("red"), |_| BallColor::Red),
            map(tag("green"), |_| BallColor::Green),
            map(tag("blue"), |_| BallColor::Blue),
        ))(i)?;
        Ok((i, (n, color)))
    }

    pub fn parse(input: &str) -> IResult<&str, BallSet> {
        let (i, groups) = separated_list1(char(','), Self::parse_ball_group)(input)?;
        let mut set = BallSet::default();
        for (n, color) in groups {
            match color {
                BallColor::Red => set.red_cnt += n,
                BallColor::Green => set.green_cnt += n,
                BallColor::Blue => set.blue_cnt += n,
            }
        }
        Ok((i, set))
    }

    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.red_cnt, self.green_cnt, self.blue_cnt)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<BallSet>,
}
impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        let (i, _) = tag("Game ")(input)?;
        let (i, id) = u32(i)?;
        let (i, _) = char(':')(i)?;
        let (i, sets) = separated_list1(char(';'), BallSet::parse)(i)?;
        Ok((i, Game { id, sets }))
    }
    pub fn possible_to_have(&self, red: u32, green: u32, blue: u32) -> bool {
        for set in &self.sets {
            if set.red_cnt > red || set.green_cnt > green || set.blue_cnt > blue {
                return false;
            }
        }
        true
    }

    pub fn smallest_ball_set(&self) -> BallSet {
        let mut res = BallSet::default();
        for set in &self.sets {
            res.red_cnt = res.red_cnt.max(set.red_cnt);
            res.green_cnt = res.green_cnt.max(set.green_cnt);
            res.blue_cnt = res.blue_cnt.max(set.blue_cnt);
        }
        res
    }
}

pub fn day2_1(input: &str) -> Result<u32, ParseError> {
    input
        .lines()
        .map(|s| Ok(Game::parse(s)?.1))
        .filter_ok(|game| game.possible_to_have(12, 13, 14))
        .map_ok(|game| game.id)
        .fold_ok(0, |acc, n| acc + n)
}

pub fn day2_2(input: &str) -> Result<u32, ParseError> {
    input
        .lines()
        .map(|s| Ok(Game::parse(s)?.1))
        .map_ok(|game| game.smallest_ball_set().to_tuple())
        .map_ok(|(r, g, b)| r * g * b)
        .fold_ok(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::day2_1;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_day2_1() {
        let res = day2_1(INPUT);
        assert_eq!(res, Ok(8))
    }

    #[test]
    fn test_day2_2() {
        let res = day2_1(INPUT);
        assert_eq!(res, Ok(8))
    }
}
