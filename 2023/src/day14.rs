use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::grid::{AsciiGrid, ColumnIterator, Direction, Grid, LineIterator, MutGrid, VecGrid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn from_ascii(c: u8) -> Self {
        match c {
            b'O' => Self::RoundedRock,
            b'#' => Self::CubeRock,
            b'.' => Self::Empty,
            _ => unimplemented!(),
        }
    }
}

fn total_load(platform: &VecGrid<Tile>) -> usize {
    platform
        .lines()
        .enumerate()
        .map(|(y, line_iter)| {
            let num_rocks = line_iter.filter(|tile| tile == &&Tile::RoundedRock).count();
            num_rocks * (platform.height() - y)
        })
        .sum()
}

#[derive(Debug)]
struct Range {
    x0: usize,
    x1: usize,
    num_rocks: usize,
}

fn slide(platform: &mut VecGrid<Tile>, dir: Direction) {
    let mut ranges = vec![];
    let traversal_range = if dir.is_vertical() {
        platform.x0()..=platform.x1()
    } else {
        platform.y0()..=platform.y1()
    };
    for line in traversal_range {
        ranges.clear();
        if dir.is_vertical() {
            collect_ranges(platform, ColumnIterator::new(platform, line), &mut ranges);
        } else {
            collect_ranges(platform, LineIterator::new(platform, line), &mut ranges);
        };
        for range in &ranges {
            let range_size = range.x1 - range.x0 + 1;
            if range.num_rocks == 0 || range.num_rocks == range_size {
                continue;
            }

            let rocks_iter = std::iter::repeat_n(Tile::RoundedRock, range.num_rocks);
            let empty_iter = std::iter::repeat_n(Tile::Empty, range_size - range.num_rocks);

            let new_range = if matches!(dir, Direction::Up | Direction::Left) {
                rocks_iter.chain(empty_iter)
            } else {
                empty_iter.chain(rocks_iter)
            };

            if dir.is_vertical() {
                platform.set_vertical_range(line, range.x0, range.x1, new_range);
            } else {
                platform.set_horizontal_range(line, range.x0, range.x1, new_range);
            }
        }
    }
}

fn collect_ranges<'a>(
    platform: &'a VecGrid<Tile>,
    iter: impl Iterator<Item = &'a Tile>,
    ranges: &mut Vec<Range>,
) {
    let mut num_rocks = 0;
    let mut x0 = 0;
    for (i, tile) in iter.enumerate() {
        match tile {
            Tile::RoundedRock => {
                num_rocks += 1;
            }
            Tile::Empty => {}
            Tile::CubeRock => {
                if i != x0 && num_rocks > 0 {
                    ranges.push(Range {
                        x0,
                        x1: i - 1,
                        num_rocks,
                    });
                }
                x0 = i + 1;
                num_rocks = 0;
            }
        }
    }
    if num_rocks != 0 {
        ranges.push(Range {
            x0,
            x1: platform.x1(),
            num_rocks,
        });
    }
}

fn process_input(input: &str) -> VecGrid<Tile> {
    let ascii_grid = AsciiGrid::from_ascii(input.as_bytes());
    VecGrid::from_iter(
        ascii_grid.width(),
        ascii_grid.height(),
        ascii_grid.iter().map(|&c| Tile::from_ascii(c)),
    )
}

pub fn day14_1(input: &str) -> Result<usize, ()> {
    let mut platform = process_input(input);
    slide(&mut platform, Direction::Up);
    Ok(total_load(&platform))
}

fn cycle(platform: &mut VecGrid<Tile>) {
    slide(platform, Direction::Up);
    slide(platform, Direction::Left);
    slide(platform, Direction::Down);
    slide(platform, Direction::Right);
}

pub fn day14_2(input: &str) -> Result<usize, ()> {
    let mut platform = process_input(input);
    let mut remaining_cycles = 1_000_000_000usize;
    let mut seen_states: HashMap<u64, usize> = HashMap::new();
    while remaining_cycles != 0 {
        remaining_cycles -= 1;
        cycle(&mut platform);
        let mut s = DefaultHasher::new();
        platform.hash(&mut s);
        let hash = s.finish();
        if let std::collections::hash_map::Entry::Vacant(e) = seen_states.entry(hash) {
            e.insert(remaining_cycles);
        } else {
            let last_seen = seen_states
                .get(&hash)
                .expect("Map contains key in this branch");
            let diff = last_seen - remaining_cycles;
            remaining_cycles %= diff;
            break;
        }
    }
    while remaining_cycles != 0 {
        remaining_cycles -= 1;
        cycle(&mut platform);
    }
    Ok(total_load(&platform))
}

#[cfg(test)]
mod tests {
    use super::{day14_1, day14_2};

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_day14_1() {
        let res = day14_1(INPUT);
        assert_eq!(res, Ok(136))
    }

    #[test]
    fn test_day14_2() {
        let res = day14_2(INPUT);

        assert_eq!(res, Ok(64))
    }
}
