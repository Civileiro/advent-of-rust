#![allow(dead_code)]

use std::{collections::HashSet, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord3d {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord3d {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Grid3d<T> {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub grid_raw: Vec<T>,
}

impl<T> Grid3d<T> {
    pub fn from_vec(width: usize, height: usize, depth: usize, grid_raw: Vec<T>) -> Self {
        assert_eq!(width * height * depth, grid_raw.len());
        Self {
            width,
            height,
            depth,
            grid_raw,
        }
    }
    pub fn get_unchecked(&self, c: &Coord3d) -> &T {
        debug_assert!(c.x < self.width);
        debug_assert!(c.y < self.height);
        debug_assert!(c.z < self.depth);
        &self.grid_raw[c.z * self.width * self.height + c.y * self.width + c.x]
    }
    pub fn get_mut_unchecked(&mut self, c: &Coord3d) -> &mut T {
        debug_assert!(c.x < self.width);
        debug_assert!(c.y < self.height);
        debug_assert!(c.z < self.depth);
        &mut self.grid_raw[c.z * self.width * self.height + c.y * self.width + c.x]
    }
}

impl FromStr for Coord3d {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(|n| n.parse::<usize>());
        Ok(Self {
            x: nums.next().unwrap()? + 1,
            y: nums.next().unwrap()? + 1,
            z: nums.next().unwrap()? + 1,
        })
    }
}

fn parse_input(input: &str) -> Vec<Coord3d> {
    input
        .lines()
        .map(Coord3d::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn day18_1(input: &str) -> usize {
    let coords = parse_input(input);
    let mut field = Grid3d::from_vec(22, 22, 22, vec![false; 22 * 22 * 22]);
    let mut surface_area = 0;
    for c in coords {
        *field.get_mut_unchecked(&c) = true;
        surface_area += 6;
        for x in [c.x - 1, c.x + 1] {
            if *field.get_unchecked(&Coord3d { x, y: c.y, z: c.z }) {
                surface_area -= 2;
            }
        }
        for y in [c.y - 1, c.y + 1] {
            if *field.get_unchecked(&Coord3d { x: c.x, y, z: c.z }) {
                surface_area -= 2;
            }
        }
        for z in [c.z - 1, c.z + 1] {
            if *field.get_unchecked(&Coord3d { x: c.x, y: c.y, z }) {
                surface_area -= 2;
            }
        }
    }
    surface_area
}

pub fn day18_2(input: &str) -> usize {
    let coords = parse_input(input);
    let mut field = Grid3d::from_vec(22, 22, 22, vec![false; 22 * 22 * 22]);
    for c in coords {
        *field.get_mut_unchecked(&c) = true;
    }
    let mut surface_area = 0;
    let mut coords_next = Vec::with_capacity(20 * 20 * 20);
    let mut visited: HashSet<Coord3d> = HashSet::with_capacity(field.grid_raw.len());
    coords_next.push(Coord3d::new(0, 0, 0));
    while let Some(c) = coords_next.pop() {
        if visited.contains(&c) {
            continue;
        }
        visited.insert(c.clone());
        if c.x != 0 {
            let cx = Coord3d::new(c.x - 1, c.y, c.z);
            if *field.get_unchecked(&cx) {
                surface_area += 1
            } else {
                coords_next.push(cx)
            }
        }
        if c.x + 1 < field.width {
            let cx = Coord3d::new(c.x + 1, c.y, c.z);
            if *field.get_unchecked(&cx) {
                surface_area += 1
            } else {
                coords_next.push(cx)
            }
        }
        if c.y != 0 {
            let cy = Coord3d::new(c.x, c.y - 1, c.z);
            if *field.get_unchecked(&cy) {
                surface_area += 1
            } else {
                coords_next.push(cy)
            }
        }
        if c.y + 1 < field.height {
            let cy = Coord3d::new(c.x, c.y + 1, c.z);
            if *field.get_unchecked(&cy) {
                surface_area += 1
            } else {
                coords_next.push(cy)
            }
        }
        if c.z != 0 {
            let cz = Coord3d::new(c.x, c.y, c.z - 1);
            if *field.get_unchecked(&cz) {
                surface_area += 1
            } else {
                coords_next.push(cz)
            }
        }
        if c.z + 1 < field.depth {
            let cz = Coord3d::new(c.x, c.y, c.z + 1);
            if *field.get_unchecked(&cz) {
                surface_area += 1
            } else {
                coords_next.push(cz)
            }
        }
    }
    surface_area
}

const _TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
