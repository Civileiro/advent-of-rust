#![allow(dead_code)]

use crate::grid::Grid;
use lazy_static::lazy_static;

use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
struct Rock {
    shape: Grid<bool>,
}

impl Rock {
    pub fn new_dash() -> Self {
        let shape = Grid::from_vec(4, 1, vec![true; 4]);
        Self { shape }
    }
    pub fn new_plus() -> Self {
        let shape = Grid::from_vec(
            3,
            3,
            vec![false, true, false, true, true, true, false, true, false],
        );
        Self { shape }
    }
    pub fn new_l() -> Self {
        let shape = Grid::from_vec(
            3,
            3,
            vec![true, true, true, false, false, true, false, false, true],
        );
        Self { shape }
    }
    pub fn new_i() -> Self {
        let shape = Grid::from_vec(1, 4, vec![true; 4]);
        Self { shape }
    }
    pub fn new_square() -> Self {
        let shape = Grid::from_vec(2, 2, vec![true; 4]);
        Self { shape }
    }
}

lazy_static! {
    static ref FIELD_ROCKS: [Rock; 5] = [
        Rock::new_dash(),
        Rock::new_plus(),
        Rock::new_l(),
        Rock::new_i(),
        Rock::new_square(),
    ];
}
#[derive(Debug)]
enum Wind {
    Left,
    Right,
}

struct Cycler<'a, T> {
    content: &'a [T],
    index: usize,
}

impl<'a, T> Cycler<'a, T> {
    pub fn new(content: &'a [T]) -> Self {
        Self { content, index: 0 }
    }
    pub fn next(&mut self) -> &'a T {
        let res = &self.content[self.index];
        self.index = (self.index + 1) % self.content.len();
        res
    }
}

struct Field<'a> {
    rock_cycle: Cycler<'a, Rock>,
    wind_cycle: Cycler<'a, Wind>,
    field: Grid<bool>,
    aprox_height: usize,
}

impl<'a> Debug for Field<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for y in (0..8).rev() {
            for c in self
                .field
                .get_line(y)
                .iter()
                .map(|b| if *b { '#' } else { '.' })
            {
                f.write_fmt(format_args!("{c}"))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<'a> Field<'a> {
    pub fn new(wind: &'a [Wind]) -> Self {
        let field = Grid::from_vec(7, 10000, vec![false; 7 * 10000]);
        Self {
            rock_cycle: Cycler::new(FIELD_ROCKS.as_slice()),
            wind_cycle: Cycler::new(wind),
            field,
            aprox_height: 0,
        }
    }
    pub fn height(&self) -> usize {
        for y in self.aprox_height.. {
            if self.field.get_line(y).iter().all(|b| !b) {
                return y;
            }
        }
        unreachable!()
    }
    pub fn drop_rock(&mut self) {
        let mut x: usize = 2;
        let mut y: usize = self.height() + 3;
        let rock = self.rock_cycle.next();
        loop {
            {
                let nx = match self.wind_cycle.next() {
                    Wind::Left => x.saturating_sub(1),
                    Wind::Right => x + 1,
                };
                if !self.overlaps(&rock.shape, nx, y) {
                    x = nx
                }
            }
            if y == 0 || self.overlaps(&rock.shape, x, y - 1) {
                self.aprox_height = y;
                self.record(&rock.shape, x, y);
                break;
            } else {
                y -= 1
            }
        }
    }
    fn overlaps(&self, shape: &Grid<bool>, x: usize, y: usize) -> bool {
        if x + shape.width > 7 {
            return true;
        }
        for xi in 0..shape.width {
            for yi in 0..shape.height {
                if *shape.get(xi, yi) && *self.field.get(x + xi, y + yi) {
                    return true;
                }
            }
        }
        false
    }
    fn record(&mut self, shape: &Grid<bool>, x: usize, y: usize) {
        for xi in 0..shape.width {
            for yi in 0..shape.height {
                if *shape.get(xi, yi) {
                    *self.field.get_mut(x + xi, y + yi) = true;
                }
            }
        }
    }
    pub fn height_line(&self) -> [u8; 7] {
        let height = self.height() - 1;
        let mut res: [Option<u8>; 7] = [None; 7];
        for line_index in 0.. {
            if height < line_index {
                res.iter_mut()
                    .filter(|r| r.is_none())
                    .for_each(|r| *r = Some(line_index as u8));
                break;
            }
            let line = self.field.get_line(height - line_index);
            for i in 0..line.len() {
                if res[i].is_none() && line[i] {
                    res[i] = Some(line_index as u8)
                }
            }
        }
        let mut real_res = [0; 7];
        for (i, r) in res.into_iter().enumerate() {
            real_res[i] = r.unwrap()
        }
        real_res
    }
    pub fn state(&self) -> FieldState {
        FieldState::new(self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FieldState {
    rock_index: usize,
    wind_index: usize,
    height_line: [u8; 7],
}

impl FieldState {
    pub fn new(field: &Field) -> Self {
        Self {
            rock_index: field.rock_cycle.index,
            wind_index: field.wind_cycle.index,
            height_line: field.height_line(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Wind> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Wind::Right,
            '<' => Wind::Left,
            _ => unimplemented!(),
        })
        .collect()
}

pub fn day17_1(input: &str) -> usize {
    let wind = parse_input(input);
    let mut field = Field::new(&wind);
    for _ in 0..2022 {
        field.drop_rock();
    }
    field.height()
}

fn get_first_repetition(field: &mut Field) -> ((usize, usize), (usize, usize)) {
    let mut found_states: HashMap<FieldState, Vec<(usize, usize)>> = HashMap::new();

    for i in 0.. {
        field.drop_rock();
        match found_states.entry(field.state()) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.get_mut().push((i, field.aprox_height));
                let rep = e.get();
                if rep.len() >= 2 {
                    let ((i1, h1), (i2, h2)) = (rep[0], rep[1]);
                    let adjust = field.height() - h2;
                    return ((i1, h1 + adjust), (i2, h2 + adjust));
                }
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(vec![(i, field.aprox_height)]);
            }
        }
    }
    unreachable!()
}

pub fn day17_2(input: &str) -> usize {
    let wind = parse_input(input);
    let mut field = Field::new(&wind);
    let ((i1, h1), (i2, h2)) = get_first_repetition(&mut field);

    let cycles_remaining = (1_000_000_000_000 - i2) / (i2 - i1);
    let height_skip = cycles_remaining * (h2 - h1);
    let i_skip = cycles_remaining * (i2 - i1);

    for _ in (i2 + i_skip + 1)..1_000_000_000_000 {
        field.drop_rock();
    }

    field.height() + height_skip
} // 1525364434259 wrong

const _TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
