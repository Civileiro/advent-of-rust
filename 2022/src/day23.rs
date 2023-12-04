#![allow(dead_code)]

use crate::grid::Grid;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Proposal {
    Propose(Coord),
    // Empty,
    Overlaped,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

type Coord = (usize, usize);

impl Dir {
    pub fn possible_coords(&self, (x, y): Coord) -> [Coord; 3] {
        match self {
            Dir::North => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
            Dir::South => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
            Dir::West => [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
            Dir::East => [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        }
    }
    pub fn move_coord(&self, (x, y): Coord) -> Coord {
        match self {
            Dir::North => (x, y - 1),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y),
            Dir::East => (x + 1, y),
        }
    }
}

fn parse_input(input: &str) -> (Grid<bool>, (Coord, Coord)) {
    let multiplier = 3;
    let width = input.lines().next().unwrap().len() * multiplier;
    let height = input.chars().filter(|c| c == &'\n').count() * multiplier;
    let raw_grid = vec![false; width * height];

    let mut grid = Grid::from_vec(width, height, raw_grid);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            *grid.get_mut(x + width / 3, y + height / 3) = match c {
                '.' => false,
                '#' => true,
                _ => unimplemented!(),
            }
        }
    }

    (
        grid,
        ((width / 3, height / 3), (2 * width / 3, 2 * height / 3)),
    )
}

fn around_coord((x, y): Coord) -> [Coord; 8] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn first_half(
    elves: &Grid<bool>,
    proposes: &mut HashMap<Coord, Proposal>,
    directions: &[Dir; 4],
    ((x1, y1), (x2, y2)): (Coord, Coord),
) -> (Coord, Coord) {
    let mut min_x: usize = usize::MAX;
    let mut min_y: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for y in y1..=y2 {
        for x in x1..=x2 {
            if !*elves.get(x, y) {
                continue;
            }
            if x < min_x {
                min_x = x
            }
            if x > max_x {
                max_x = x
            }
            if y < min_y {
                min_y = y
            }
            if y > max_y {
                max_y = y
            }
            if !around_coord((x, y))
                .into_iter()
                .any(|(dx, dy)| *elves.get(dx, dy))
            {
                continue;
            }
            let proposal_dir = directions.iter().find(|d| {
                d.possible_coords((x, y))
                    .into_iter()
                    .all(|(px, py)| !elves.get(px, py))
            });
            if let Some(dir) = proposal_dir {
                // println!("elf {x} {y} is trying to go {dir:?}");
                let pc = dir.move_coord((x, y));
                use std::collections::hash_map::Entry;
                match proposes.entry(pc) {
                    Entry::Occupied(e) => *e.into_mut() = Proposal::Overlaped,
                    Entry::Vacant(e) => {
                        e.insert(Proposal::Propose((x, y)));
                    }
                }
            }
        }
    }
    ((min_x - 1, min_y - 1), (max_x + 1, max_y + 1))
}

fn second_half(elves: &mut Grid<bool>, proposes: &HashMap<Coord, Proposal>) -> bool {
    let mut elf_moved = false;
    for (&(x, y), &p) in proposes {
        if let Proposal::Propose((px, py)) = p {
            elf_moved = true;
            *elves.get_mut(px, py) = false;
            *elves.get_mut(x, y) = true;
        }
    }
    elf_moved
}

fn smallest_rectangle(elves: &Grid<bool>) -> (Coord, Coord) {
    let mut min_x: usize = usize::MAX;
    let mut min_y: usize = usize::MAX;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for y in 0..elves.height {
        for x in 0..elves.width {
            if !*elves.get(x, y) {
                continue;
            }
            if x < min_x {
                min_x = x
            }
            if x > max_x {
                max_x = x
            }
            if y < min_y {
                min_y = y
            }
            if y > max_y {
                max_y = y
            }
        }
    }
    ((min_x, min_y), (max_x, max_y))
}

pub fn day23_1(input: &str) -> usize {
    let (mut elves, mut search_area) = parse_input(input);
    let mut proposes: HashMap<Coord, Proposal> = HashMap::new();
    let mut directions = [Dir::North, Dir::South, Dir::West, Dir::East];
    for _ in 0..10 {
        proposes.clear();
        search_area = first_half(&elves, &mut proposes, &directions, search_area);
        second_half(&mut elves, &proposes);
        directions.as_mut_slice().rotate_left(1);
    }

    let ((x1, y1), (x2, y2)) = smallest_rectangle(&elves);
    let mut empty_groud = 0;
    for y in y1..=y2 {
        for x in x1..=x2 {
            if !*elves.get(x, y) {
                empty_groud += 1
            }
        }
    }

    empty_groud
} // 3766

pub fn day23_2(input: &str) -> usize {
    let (mut elves, mut search_area) = parse_input(input);
    let mut proposes: HashMap<Coord, Proposal> = HashMap::new();
    let mut directions = [Dir::North, Dir::South, Dir::West, Dir::East];
    for round in 1.. {
        proposes.clear();
        search_area = first_half(&elves, &mut proposes, &directions, search_area);
        let elf_moved = second_half(&mut elves, &proposes);
        if !elf_moved {
            return round;
        }
        directions.as_mut_slice().rotate_left(1);
    }
    unreachable!()
} // 954

const _TEST_INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
