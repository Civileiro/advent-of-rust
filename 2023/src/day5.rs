use std::ops::Range;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::value,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::parse::sp;

pub type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

type Id = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Category {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Seed, tag("seed")),
            value(Self::Soil, tag("soil")),
            value(Self::Fertilizer, tag("fertilizer")),
            value(Self::Water, tag("water")),
            value(Self::Light, tag("light")),
            value(Self::Temperature, tag("temperature")),
            value(Self::Humidity, tag("humidity")),
            value(Self::Location, tag("location")),
        ))(input)
    }
}

pub fn parse_seed_list(input: &str) -> IResult<&str, Vec<Id>> {
    let (i, _) = preceded(sp, tag("seeds:"))(input)?;
    separated_list1(char(' '), preceded(sp, u64))(i)
}

#[derive(Debug)]
struct MapRange {
    source_start: Id,
    destination_start: Id,
    range_length: Id,
}

impl MapRange {
    pub fn parse(input: &str) -> IResult<&str, MapRange> {
        let (i, destination_start) = u64(input)?;
        let (i, _) = char(' ')(i)?;
        let (i, source_start) = u64(i)?;
        let (i, _) = char(' ')(i)?;
        let (i, range_length) = u64(i)?;
        Ok((
            i,
            Self {
                destination_start,
                source_start,
                range_length,
            },
        ))
    }

    pub fn convert(&self, id: Id) -> Option<Id> {
        if (self.source_start..(self.source_start + self.range_length)).contains(&id) {
            Some(id - self.source_start + self.destination_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CategoryMap {
    source_category: Category,
    destination_category: Category,
    ranges: Vec<MapRange>,
}

impl CategoryMap {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (i, source_category) = preceded(sp, Category::parse)(input)?;
        let (i, _) = tag("-to-")(i)?;
        let (i, destination_category) = Category::parse(i)?;
        let (i, _) = tag(" map:\n")(i)?;
        let (i, mut ranges) = separated_list1(char('\n'), MapRange::parse)(i)?;
        ranges.sort_unstable_by_key(|range| range.source_start);
        Ok((
            i,
            Self {
                source_category,
                destination_category,
                ranges,
            },
        ))
    }

    pub fn convert(&self, id: Id) -> Id {
        for range in &self.ranges {
            let Some(convertion) = range.convert(id) else {
                continue;
            };
            return convertion;
        }
        id
    }

    pub fn convert_range(&self, mut range: Range<Id>) -> Vec<Range<Id>> {
        let mut res = Vec::new();
        let mut push_to_res = |range: Range<Id>| {
            if range.end > range.start {
                res.push(range)
            }
        };
        let init = self.ranges.partition_point(|range_map| {
            range_map.source_start + range_map.range_length <= range.start
        });
        for range_map in &self.ranges[init..] {
            if range.end < range_map.source_start {
                break;
            }
            let source_end = range_map.source_start + range_map.range_length;
            let destination_end = range_map.destination_start + range_map.range_length;
            match (
                range_map.source_start < range.start,
                range.end <= source_end,
            ) {
                // mapped range is inside range
                (false, false) => {
                    // range before map
                    let range_before = range.start..range_map.source_start;
                    push_to_res(range_before);
                    // range being mapped
                    let mapped_range = range_map.destination_start..destination_end;
                    push_to_res(mapped_range);

                    range.start = source_end;
                }
                // mapped range continues to the right
                (false, true) => {
                    // range before map
                    let range_before = range.start..range_map.source_start;
                    push_to_res(range_before);
                    // range being mapped goes from `map start` to `range end`
                    let mapped_range_length = range.end - range_map.source_start;
                    let destination_end = range_map.destination_start + mapped_range_length;
                    let mapped_range = range_map.destination_start..destination_end;
                    push_to_res(mapped_range);

                    range.end = range.start;
                    break;
                }
                // mapped range continues to the left
                (true, false) => {
                    // range being mapped goes from `range start` to `map end`
                    let mapped_range_length = source_end - range.start;
                    let destination_start = destination_end - mapped_range_length;
                    let mapped_range = destination_start..destination_end;
                    push_to_res(mapped_range);

                    range.start = source_end;
                }
                // mapped range includes range
                (true, true) => {
                    let destination_start =
                        range_map.destination_start + range.start - range_map.source_start;
                    let destination_end = destination_start + range.end - range.start;
                    let mapped_range = destination_start..destination_end;
                    push_to_res(mapped_range);

                    range.end = range.start;
                    break;
                }
            }
        }
        push_to_res(range);
        res
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Id>, Vec<CategoryMap>)> {
    let (i, seeds) = parse_seed_list(input)?;
    let (i, maps) = separated_list1(tag("\n\n"), CategoryMap::parse)(i)?;
    Ok((i, (seeds, maps)))
}

pub fn day5_1(input: &str) -> Result<Id, ParseError> {
    let (_, (mut ids, maps)) = parse_input(input)?;
    let mut curr_cat = Category::Seed;
    while curr_cat != Category::Location {
        let map = maps
            .iter()
            .find(|map| map.source_category == curr_cat)
            .unwrap();
        for id in &mut ids {
            *id = map.convert(*id);
        }
        curr_cat = map.destination_category;
    }
    Ok(ids.into_iter().min().unwrap())
}

pub fn day5_2(input: &str) -> Result<Id, ParseError> {
    let (_, (ranges, maps)) = parse_input(input)?;
    let mut ranges: Vec<_> = ranges
        .into_iter()
        .tuples()
        .map(|(begin, length)| begin..(begin + length))
        .collect();
    let mut ranges_aux = Vec::with_capacity(ranges.capacity());

    let mut curr_cat = Category::Seed;
    while curr_cat != Category::Location {
        let map = maps
            .iter()
            .find(|map| map.source_category == curr_cat)
            .unwrap();

        ranges_aux.extend(
            ranges
                .iter()
                .flat_map(|range| map.convert_range(range.clone())),
        );
        ranges.clear();
        std::mem::swap(&mut ranges, &mut ranges_aux);

        curr_cat = map.destination_category;
    }

    let smallest = ranges.into_iter().min_by_key(|range| range.start).unwrap();

    Ok(smallest.start)
}

#[cfg(test)]
mod tests {

    use super::{day5_1, day5_2};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_day5_1() {
        let res = day5_1(INPUT);
        assert_eq!(res, Ok(35))
    }

    #[test]
    fn test_day5_2() {
        let res = day5_2(INPUT);
        assert_eq!(res, Ok(46))
    }
}
