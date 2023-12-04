#![allow(dead_code)]

use itertools::Itertools;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Line {
    pub a: Coord,
    pub b: Coord,
}

impl Line {
    pub fn new(a @ (x1, y1): Coord, b @ (x2, y2): Coord) -> Self {
        debug_assert!(x1 == x2 || y1 == y2);
        Self { a, b }
    }
}

fn add_rocks(grid: &mut Grid<Tile>, line: Line) {
    let (x1, y1) = line.a;
    let (x2, y2) = line.b;
    let x_range = if x1 > x2 { x2..=x1 } else { x1..=x2 };
    let y_range = if y1 > y2 { y2..=y1 } else { y1..=y2 };
    for x in x_range {
        for y in y_range.clone() {
            *grid.get_mut(x, y) = Tile::Rock;
        }
    }
}

fn parse_input(input: &str) -> (Grid<Tile>, usize) {
    let lines = input.lines().flat_map(|line| {
        line.split("->")
            .map(|coord| {
                let (x, y) = coord.trim().split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .tuple_windows()
            .map(|(c1, c2)| Line::new(c1, c2))
    });
    let mut grid = Grid::from_vec(1000, 200, vec![Tile::Air; 200_000]);
    let mut lowest = 0;
    for l in lines {
        add_rocks(&mut grid, l);
        if l.a.1 > lowest {
            lowest = l.a.1;
        }
        if l.b.1 > lowest {
            lowest = l.b.1;
        }
    }
    (grid, lowest)
}

fn pour_sand(grid: &mut Grid<Tile>) {
    while let Some((x, y)) = next_sand_spot(grid) {
        *grid.get_mut(x, y) = Tile::Sand
    }
}

fn next_sand_spot(grid: &Grid<Tile>) -> Option<Coord> {
    let (mut x, mut y) = (500, 0);
    if grid.get(x, y) == &Tile::Sand {
        return None;
    }
    while y + 1 < grid.height {
        if grid.get(x, y + 1) == &Tile::Air {
            y += 1;
            continue;
        } else if grid.get(x - 1, y + 1) == &Tile::Air {
            y += 1;
            x -= 1;
            continue;
        } else if grid.get(x + 1, y + 1) == &Tile::Air {
            y += 1;
            x += 1;
            continue;
        }
        return Some((x, y));
    }
    None
}

pub fn day14_1(input: &str) -> usize {
    let (mut grid, _) = parse_input(input);
    pour_sand(&mut grid);
    grid.grid_raw
        .into_iter()
        .filter(|t| t == &Tile::Sand)
        .count()
}

fn add_floor(grid: &mut Grid<Tile>, y: usize) {
    for x in 0..1000 {
        *grid.get_mut(x, y) = Tile::Rock
    }
}

pub fn day14_2(input: &str) -> usize {
    let (mut grid, lowest) = parse_input(input);
    add_floor(&mut grid, lowest + 2);
    pour_sand(&mut grid);
    grid.grid_raw
        .into_iter()
        .filter(|t| t == &Tile::Sand)
        .count()
}

const _TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
