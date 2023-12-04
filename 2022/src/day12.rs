#![allow(dead_code)]

use arrayvec::ArrayVec;

use crate::grid::Grid;

type CoordU = u32;
type GridU = i32;

fn find_start_end(grid: &Grid<char>) -> ((CoordU, CoordU), (CoordU, CoordU)) {
    let mut start: Option<(CoordU, CoordU)> = None;
    let mut end: Option<(CoordU, CoordU)> = None;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == &'S' {
                start = Some((x as CoordU, y as CoordU));
            } else if grid.get(x, y) == &'E' {
                end = Some((x as CoordU, y as CoordU))
            }
        }
    }
    (start.unwrap(), end.unwrap())
}

fn input_to_grid(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid_raw = input.lines().flat_map(|line| line.chars()).collect();
    Grid::from_vec(width, height, grid_raw)
}

fn neighbors(grid: &Grid<GridU>, (x, y): (CoordU, CoordU)) -> ArrayVec<(CoordU, CoordU), 4> {
    let mut neighbors = ArrayVec::new();
    let mut push_if_able = |xn: CoordU, yn: CoordU| {
        let curr = grid.get(x as usize, y as usize);
        let to_go = grid.get(xn as usize, yn as usize);
        if *curr >= to_go - 1 {
            neighbors.push((xn, yn));
        }
    };

    if x != 0 {
        push_if_able(x - 1, y)
    }
    if x + 1 < grid.width as CoordU {
        push_if_able(x + 1, y)
    }
    if y != 0 {
        push_if_able(x, y - 1)
    }
    if y + 1 < grid.height as CoordU {
        push_if_able(x, y + 1)
    }
    neighbors
}

fn bfs_length(grid: &Grid<GridU>, start: (CoordU, CoordU), end: (CoordU, CoordU)) -> usize {
    let mut length = 0;
    let mut to_explore = std::collections::VecDeque::new();
    let mut to_explore_next = to_explore.clone();
    to_explore.push_back(start);
    let mut marked = Grid::from_vec(
        grid.width,
        grid.height,
        grid.grid_raw.iter().map(|_| false).collect(),
    );

    loop {
        while let Some(coord) = to_explore.pop_front() {
            if coord == end {
                return length;
            }
            for n in neighbors(grid, coord) {
                if !marked.get(n.0 as usize, n.1 as usize) {
                    to_explore_next.push_back(n);
                    *marked.get_mut(n.0 as usize, n.1 as usize) = true;
                }
            }
        }
        length += 1;
        core::mem::swap(&mut to_explore, &mut to_explore_next);
    }
}

fn char_grid_to_int(grid: Grid<char>) -> Grid<GridU> {
    Grid::from_vec(
        grid.width,
        grid.height,
        grid.grid_raw
            .into_iter()
            .map(|c| match c {
                'S' => 'a' as GridU,
                'E' => 'z' as GridU,
                'a'..='z' => c as GridU,
                _ => unimplemented!(),
            })
            .collect(),
    )
}

pub fn day12_1(input: &str) -> usize {
    let grid = input_to_grid(input);
    let (start, end) = find_start_end(&grid);
    let grid = char_grid_to_int(grid);
    bfs_length(&grid, start, end)
}

fn neighbors_rev(grid: &Grid<GridU>, (x, y): (CoordU, CoordU)) -> ArrayVec<(CoordU, CoordU), 4> {
    let mut neighbors = ArrayVec::new();
    let mut push_if_able = |xn: CoordU, yn: CoordU| {
        let curr = grid.get(x as usize, y as usize);
        let to_go = grid.get(xn as usize, yn as usize);
        if *curr <= to_go + 1 {
            neighbors.push((xn, yn));
        }
    };

    if x != 0 {
        push_if_able(x - 1, y)
    }
    if x + 1 < grid.width as CoordU {
        push_if_able(x + 1, y)
    }
    if y != 0 {
        push_if_able(x, y - 1)
    }
    if y + 1 < grid.height as CoordU {
        push_if_able(x, y + 1)
    }
    neighbors
}

fn bfs_length_rev(grid: &Grid<GridU>, end: (CoordU, CoordU)) -> usize {
    let mut length = 0;
    let mut to_explore = std::collections::VecDeque::new();
    let mut to_explore_next = to_explore.clone();
    to_explore.push_back(end);
    let mut marked = Grid::from_vec(
        grid.width,
        grid.height,
        grid.grid_raw.iter().map(|_| false).collect(),
    );

    loop {
        while let Some(coord) = to_explore.pop_front() {
            if *grid.get(coord.0 as usize, coord.1 as usize) == 'a' as i32 {
                return length;
            }
            for n in neighbors_rev(grid, coord) {
                if !marked.get(n.0 as usize, n.1 as usize) {
                    to_explore_next.push_back(n);
                    *marked.get_mut(n.0 as usize, n.1 as usize) = true;
                }
            }
        }
        length += 1;
        core::mem::swap(&mut to_explore, &mut to_explore_next);
    }
}

pub fn day12_2(input: &str) -> usize {
    let grid = input_to_grid(input);
    let (_, end) = find_start_end(&grid);
    let grid = char_grid_to_int(grid);
    bfs_length_rev(&grid, end)
}

const _TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
