#![allow(dead_code, clippy::from_over_into)]
use crate::grid::Grid;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Blizzard {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Blizzard {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Self {
                up: true,
                down: false,
                left: false,
                right: false,
            },
            'v' => Self {
                up: false,
                down: true,
                left: false,
                right: false,
            },
            '<' => Self {
                up: false,
                down: false,
                left: true,
                right: false,
            },
            '>' => Self {
                up: false,
                down: false,
                left: false,
                right: true,
            },
            '.' => Self::empty(),
            c => unimplemented!("the heck is a {c}"),
        }
    }
    pub fn empty() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Bliz(Blizzard),
    Wall,
}

#[derive(Debug)]
struct State {
    start: Coord,
    end: Coord,
    grid: Grid<Tile>,
}

impl State {
    pub fn next_state(&self) -> Self {
        let mut new_grid = Grid::from_vec(
            self.grid.width,
            self.grid.height,
            vec![Tile::Wall; self.grid.width * self.grid.height],
        );
        let wrapped = |x, y| {
            let ox = if x == 0 {
                self.grid.width - 2
            } else if x == self.grid.width - 1 {
                1
            } else {
                x
            };
            let oy = if y == 0 {
                self.grid.height - 2
            } else if y == self.grid.height - 1 {
                1
            } else {
                y
            };
            (ox, oy)
        };
        for y in 1..(self.grid.height - 1) {
            for x in 1..(self.grid.width - 1) {
                match self.grid.get(x, y) {
                    Tile::Wall => (),
                    Tile::Bliz(_) => {
                        let mut b = Blizzard::empty();
                        {
                            let (ox, oy) = wrapped(x, y + 1);
                            if let Tile::Bliz(ob) = self.grid.get(ox, oy) {
                                b.up = ob.up
                            }
                        }
                        {
                            let (ox, oy) = wrapped(x, y - 1);
                            if let Tile::Bliz(ob) = self.grid.get(ox, oy) {
                                b.down = ob.down
                            }
                        }
                        {
                            let (ox, oy) = wrapped(x - 1, y);
                            if let Tile::Bliz(ob) = self.grid.get(ox, oy) {
                                b.right = ob.right
                            }
                        }
                        {
                            let (ox, oy) = wrapped(x + 1, y);
                            if let Tile::Bliz(ob) = self.grid.get(ox, oy) {
                                b.left = ob.left
                            }
                        }
                        *new_grid.get_mut(x, y) = Tile::Bliz(b)
                    }
                }
            }
        }
        *new_grid.get_mut(self.start.0, self.start.1) = Tile::Bliz(Blizzard::empty());
        *new_grid.get_mut(self.end.0, self.end.1) = Tile::Bliz(Blizzard::empty());
        State {
            start: self.start,
            end: self.end,
            grid: new_grid,
        }
    }
}

#[derive(Debug)]
struct Valley {
    start: Coord,
    end: Coord,
    repetition_num: usize,
    states: RefCell<Vec<Rc<State>>>,
}

impl Valley {
    pub fn from_input(input: &str) -> Self {
        let width = input.lines().map(|l| l.len()).max().unwrap();
        let height = input.chars().filter(|c| c == &'\n').count();
        let grid_raw = input
            .chars()
            .filter(|c| c != &'\n')
            .map(|c| match c {
                '#' => Tile::Wall,
                c => Tile::Bliz(Blizzard::from_char(c)),
            })
            .collect();
        let grid = Grid::from_vec(width, height, grid_raw);
        let start = (1, 0);
        let repetition_num = (grid.width - 2) * (grid.height - 2);
        let end = (width - 2, height - 1);
        Self {
            start,
            end,
            repetition_num,
            states: RefCell::new(vec![Rc::new(State { start, end, grid })]),
        }
    }
    pub fn get_next_coords(&self, path: &ValleyPath) -> Vec<Coord> {
        let next_state = self.get_state(path.state_index + 1);
        {
            let after_start = (self.start.0, self.start.1 + 1);
            if path.coord == self.start {
                if next_state.grid.get(after_start.0, after_start.1)
                    == &Tile::Bliz(Blizzard::empty())
                {
                    return vec![self.start, after_start];
                } else {
                    return vec![self.start];
                }
            }
        }
        {
            let after_end = (self.end.0, self.end.1 - 1);
            if path.coord == self.end {
                if next_state.grid.get(after_end.0, after_end.1) == &Tile::Bliz(Blizzard::empty()) {
                    return vec![self.end, after_end];
                } else {
                    return vec![self.end];
                }
            }
        }
        let mut nexts = Vec::new();

        for (x, y) in Self::possible_next_coord(path.coord) {
            match next_state.grid.get(x, y) {
                Tile::Wall => (),
                Tile::Bliz(b) => {
                    if b == &Blizzard::empty() {
                        nexts.push((x, y))
                    }
                }
            }
        }
        nexts
    }
    fn get_state(&self, state_index: usize) -> Rc<State> {
        let mut states = self.states.borrow_mut();
        while state_index >= states.len() {
            let next = states[states.len() - 1].next_state();
            states.push(Rc::new(next))
        }
        states[state_index].clone()
    }
    fn possible_next_coord((x, y): Coord) -> [Coord; 5] {
        [(x, y), (x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]
    }
}

#[derive(Debug, Clone, Copy)]
struct ValleyPath {
    coord: Coord,
    walked: usize,
    state_index: usize,
    cost: i64,
}

impl ValleyPath {
    pub fn new(coord: Coord, walked: usize, state_index: usize) -> Self {
        Self {
            coord,
            walked,
            state_index,
            cost: walked as i64 - coord.0 as i64 - coord.1 as i64,
        }
    }
    pub fn state_hash(&self) -> ((usize, usize), usize) {
        (self.coord, self.state_index)
    }
}

struct ValleyPathToEnd {
    p: ValleyPath,
}

impl From<ValleyPath> for ValleyPathToEnd {
    fn from(value: ValleyPath) -> Self {
        Self { p: value }
    }
}

impl Into<ValleyPath> for ValleyPathToEnd {
    fn into(self) -> ValleyPath {
        self.p
    }
}

impl PartialEq for ValleyPathToEnd {
    fn eq(&self, other: &Self) -> bool {
        self.p.cost == other.p.cost
    }
}
impl Eq for ValleyPathToEnd {}
impl PartialOrd for ValleyPathToEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.p.cost.cmp(&self.p.cost))
    }
}
impl Ord for ValleyPathToEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.p.cost.cmp(&self.p.cost)
    }
}

struct ValleyPathToStart {
    p: ValleyPath,
    inv_cost: usize,
}

impl From<ValleyPath> for ValleyPathToStart {
    fn from(value: ValleyPath) -> Self {
        Self {
            p: value,
            inv_cost: value.walked + value.coord.0 + value.coord.1,
        }
    }
}

impl Into<ValleyPath> for ValleyPathToStart {
    fn into(self) -> ValleyPath {
        self.p
    }
}

impl PartialEq for ValleyPathToStart {
    fn eq(&self, other: &Self) -> bool {
        self.inv_cost == other.inv_cost
    }
}
impl Eq for ValleyPathToStart {}
impl PartialOrd for ValleyPathToStart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.inv_cost.cmp(&self.inv_cost))
    }
}
impl Ord for ValleyPathToStart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.inv_cost.cmp(&self.inv_cost)
    }
}

fn astar<O: From<ValleyPath> + Into<ValleyPath> + Eq + Ord>(
    valley: &Valley,
    init: ValleyPath,
    end: Coord,
) -> ValleyPath {
    let mut paths = BinaryHeap::<O>::new();
    paths.push(O::from(init));
    let mut visited = HashSet::<((usize, usize), usize)>::new();
    while let Some(best) = paths.pop() {
        let best: ValleyPath = best.into();
        if visited.contains(&best.state_hash()) {
            continue;
        }
        let nexts = valley.get_next_coords(&best);
        let new_state_index = (best.state_index + 1) % valley.repetition_num;
        for coord in nexts {
            if coord == end {
                return ValleyPath::new(coord, best.walked + 1, new_state_index);
            }
            let next = ValleyPath::new(coord, best.walked + 1, new_state_index);
            paths.push(O::from(next));
        }
        visited.insert(best.state_hash());
    }
    unreachable!()
}

pub fn day24_1(input: &str) -> usize {
    let valley = Valley::from_input(input);
    let init = ValleyPath::new(valley.start, 0, 0);

    astar::<ValleyPathToEnd>(&valley, init, valley.end).walked
}
// 296 low
// 297 low

pub fn day24_2(input: &str) -> usize {
    let valley = Valley::from_input(input);
    let init = ValleyPath::new(valley.start, 0, 0);

    let walk1 = astar::<ValleyPathToEnd>(&valley, init, valley.end);
    let walk2 = astar::<ValleyPathToStart>(&valley, walk1, valley.start);
    let walk3 = astar::<ValleyPathToEnd>(&valley, walk2, valley.end);
    walk3.walked
}

const _TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
