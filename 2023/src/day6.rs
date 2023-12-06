use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, i64},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn parse_races(input: &str) -> IResult<&str, Vec<Self>> {
        let (i, times) = delimited(tag("Time:"), many1(preceded(sp, i64)), char('\n'))(input)?;
        let (i, distances) = preceded(tag("Distance:"), many1(preceded(sp, i64)))(i)?;
        let res = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Self { time, distance })
            .collect();
        Ok((i, res))
    }

    fn parse_bad_kerning_line<'a>(line_tag: &str, input: &'a str) -> IResult<&'a str, i64> {
        let (i, res) = delimited(
            tag(line_tag),
            take_while1(|c: char| c == ' ' || c.is_ascii_digit()),
            char('\n'),
        )(input)?;
        Ok((
            i,
            res.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap(),
        ))
    }

    pub fn parse_bad_kerning(input: &str) -> IResult<&str, Self> {
        let (i, time) = Self::parse_bad_kerning_line("Time:", input)?;
        let (i, distance) = Self::parse_bad_kerning_line("Distance:", i)?;
        Ok((i, Self { time, distance }))
    }

    pub fn ways_to_beat_record(&self) -> u64 {
        let t = self.time as f64;
        let d = self.distance as f64;
        let delta = (t * t - 4.0 * d).sqrt();
        let x0 = (t - delta) / 2.0;
        let x1 = (t + delta) / 2.0;
        // the final answer is the amount of integers in the range ]x0, x1[
        (x1.floor() as u64) - (x0.floor() as u64)
            // edge case when x0/x1 is an integer
            - if x0.fract() < 1e-10 { 1 } else { 0 }
    }
}

pub fn day6_1(input: &str) -> Result<u64, ParseError> {
    let (_, races) = Race::parse_races(input)?;
    let prod = races
        .into_iter()
        .map(|race| race.ways_to_beat_record())
        .product();
    Ok(prod)
}

pub fn day6_2(input: &str) -> Result<u64, ParseError> {
    let (_, race) = Race::parse_bad_kerning(input)?;
    Ok(race.ways_to_beat_record())
}

#[cfg(test)]
mod tests {
    use super::{day6_1, day6_2};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_day6_1() {
        let res = day6_1(INPUT);
        assert_eq!(res, Ok(288))
    }

    #[test]
    fn test_day6_2() {
        let res = day6_2(INPUT);
        assert_eq!(res, Ok(71503))
    }
}
