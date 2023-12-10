use crate::grid::{AsciiGrid, Coord, Direction, Grid, MutGrid, VecGrid};
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Day10Error {
    StartNotFound,
    NextPipeFromStartNotFound,
    BothSidesTouchEdge,
}

#[derive(Debug, Clone, Copy)]
struct Pipe(u8);

impl Pipe {
    fn connects_to(&self, dir: Direction) -> bool {
        if self.is_start() {
            return true;
        }
        match dir {
            Direction::Up => self.0 == b'|' || self.0 == b'L' || self.0 == b'J',
            Direction::Down => self.0 == b'|' || self.0 == b'F' || self.0 == b'7',
            Direction::Left => self.0 == b'-' || self.0 == b'7' || self.0 == b'J',
            Direction::Right => self.0 == b'-' || self.0 == b'L' || self.0 == b'F',
        }
    }
    fn is_start(&self) -> bool {
        self.0 == b'S'
    }
}

#[derive(Debug, Clone)]
struct LoopNavigator {
    pos: Coord,
    dir: Direction,
    pipe: Pipe,
}

impl LoopNavigator {
    fn next(&self, grid: &impl Grid<Item = u8>) -> Option<Self> {
        Direction::iter()
            .filter(|&dir| dir != self.dir.opposite() && self.pipe.connects_to(dir))
            .find_map(|dir| {
                let pos = self.pos.at_dir(dir)?;
                let pipe = Pipe(*grid.get_coord(pos)?);
                if !pipe.connects_to(dir.opposite()) {
                    return None;
                }
                Some(Self { pos, dir, pipe })
            })
    }
}

#[derive(Debug)]
struct LoopIterator<'a, G: Grid<Item = u8>> {
    grid: &'a G,
    navi: LoopNavigator,
}

impl<'a, G: Grid<Item = u8>> LoopIterator<'a, G> {
    fn from_grid(grid: &'a G) -> Result<Self, Day10Error> {
        let start_pos = grid
            .find_coord(|&c| Pipe(c).is_start())
            .ok_or(Day10Error::StartNotFound)?;
        let (_pos, dir, _pipe) = Direction::iter()
            .find_map(|dir| {
                let pos = start_pos.at_dir(dir)?;
                let pipe = Pipe(*grid.get_coord(pos)?);
                if !pipe.connects_to(dir.opposite()) {
                    return None;
                }
                Some((pos, dir, pipe))
            })
            .ok_or(Day10Error::NextPipeFromStartNotFound)?;
        let start_pipe = match dir {
            Direction::Up | Direction::Down => Pipe(b'|'),
            Direction::Left | Direction::Right => Pipe(b'-'),
        };
        Ok(Self {
            grid,
            navi: LoopNavigator {
                pos: start_pos,
                dir,
                pipe: start_pipe,
            },
        })
    }
}

impl<'a, G: Grid<Item = u8>> Iterator for LoopIterator<'a, G> {
    type Item = LoopNavigator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.navi.pipe.is_start() {
            return None;
        }
        self.navi = self.navi.next(self.grid)?;
        Some(self.navi.clone())
    }
}

pub fn day10_1(input: &str) -> Result<usize, Day10Error> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let loop_length = LoopIterator::from_grid(&grid)?.count();
    Ok(loop_length / 2)
}

fn dirs_to_touch(pipe: Pipe, curr_dir: Direction) -> (&'static [Direction], &'static [Direction]) {
    use Direction::*;
    match (pipe, curr_dir) {
        (Pipe(b'|'), Up) => (&[Left], &[Right]),
        (Pipe(b'|'), Down) => (&[Right], &[Left]),
        (Pipe(b'-'), Right) => (&[Up], &[Down]),
        (Pipe(b'-'), Left) => (&[Down], &[Up]),
        (Pipe(b'L'), Left) => (&[Left, Down], &[]),
        (Pipe(b'L'), Down) => (&[], &[Left, Down]),
        (Pipe(b'J'), Down) => (&[Right, Down], &[]),
        (Pipe(b'J'), Right) => (&[], &[Right, Down]),
        (Pipe(b'7'), Right) => (&[Up, Right], &[]),
        (Pipe(b'7'), Up) => (&[], &[Up, Right]),
        (Pipe(b'F'), Up) => (&[Left, Up], &[]),
        (Pipe(b'F'), Left) => (&[], &[Left, Up]),
        _ => (&[], &[]),
    }
}

/// Fills a grid and return the amount of tiles filled
/// Returns `None` if an edge is found
fn grid_fill(fill_grid: &mut impl MutGrid<Item = bool>, pos: Coord) -> Option<u32> {
    let to_fill = fill_grid.get_coord_mut(pos)?;
    if *to_fill {
        return Some(0);
    }
    *to_fill = true;
    let total_filled = 1 + Direction::iter()
        .map(|dir| grid_fill(fill_grid, pos.at_dir(dir)?))
        .fold_options(0, |acc, n| acc + n)?;
    Some(total_filled)
}

pub fn day10_2(input: &str) -> Result<u32, Day10Error> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let mut fill = VecGrid::new(grid.width(), grid.height(), false);
    for LoopNavigator { pos, .. } in LoopIterator::from_grid(&grid)? {
        *fill.get_coord_mut(pos).unwrap() = true;
    }
    let mut left_count = Some(0);
    let mut right_count = Some(0);
    for LoopNavigator { pos, dir, pipe } in LoopIterator::from_grid(&grid)? {
        let (left_dirs, right_dirs) = dirs_to_touch(pipe, dir);
        left_count = left_count.and_then(|n| {
            let fills = left_dirs
                .iter()
                .map(|&dir| grid_fill(&mut fill, pos.at_dir(dir)?))
                .fold_options(0, |acc, n| acc + n)?;
            Some(n + fills)
        });
        right_count = right_count.and_then(|n| {
            let fills = right_dirs
                .iter()
                .map(|&dir| grid_fill(&mut fill, pos.at_dir(dir)?))
                .fold_options(0, |acc, n| acc + n)?;
            Some(n + fills)
        });
    }
    [left_count, right_count]
        .into_iter()
        .flatten()
        .next()
        .ok_or(Day10Error::BothSidesTouchEdge)
}

#[cfg(test)]
mod tests {
    use super::{day10_1, day10_2};

    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
    const INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    const INPUT3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    const INPUT4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_day10_1_1() {
        let res = day10_1(INPUT1);
        assert_eq!(res, Ok(4));
    }

    #[test]
    fn test_day10_1_2() {
        let res = day10_1(INPUT2);
        assert_eq!(res, Ok(8));
    }

    #[test]
    fn test_day10_2_1() {
        let res = day10_2(INPUT3);
        assert_eq!(res, Ok(8));
    }

    #[test]
    fn test_day10_2_2() {
        let res = day10_2(INPUT4);
        assert_eq!(res, Ok(10));
    }
}
