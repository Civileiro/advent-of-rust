#![allow(dead_code)]

use crate::grid::Grid;
use either::Either;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Empty,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => Self::Empty,
            '.' => Self::Open,
            '#' => Self::Wall,
            _ => unimplemented!(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self == &Tile::Empty
    }
    pub fn is_wall(&self) -> bool {
        self == &Tile::Wall
    }
    pub fn is_open(&self) -> bool {
        self == &Tile::Open
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Turn {
    Clockwise,
    Counterclockwise,
}

impl Turn {
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => Self::Clockwise,
            'L' => Self::Counterclockwise,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn turn(&self, t: Turn) -> Self {
        match (t, self) {
            (Turn::Clockwise, Self::Left) => Self::Up,
            (Turn::Clockwise, Self::Right) => Self::Down,
            (Turn::Clockwise, Self::Up) => Self::Right,
            (Turn::Clockwise, Self::Down) => Self::Left,
            (Turn::Counterclockwise, Self::Left) => Self::Down,
            (Turn::Counterclockwise, Self::Right) => Self::Up,
            (Turn::Counterclockwise, Self::Up) => Self::Left,
            (Turn::Counterclockwise, Self::Down) => Self::Right,
        }
    }
}

type Coord = (usize, usize);
type Warp = (Coord, Dir);

struct Board {
    grid: Grid<Tile>,
    line_warp: Vec<(Option<Warp>, Option<Warp>)>,
    column_warp: Vec<(Option<Warp>, Option<Warp>)>,
    coord: (usize, usize),
    dir: Dir,
}

impl Board {
    pub fn from_str(s: &str) -> Self {
        let grid = {
            let width = s.lines().map(|l| l.len()).max().unwrap();
            let height = s.chars().filter(|c| c == &'\n').count() + 1;
            let raw_grid = vec![Tile::Empty; width * height];

            let mut grid = Grid::from_vec(width, height, raw_grid);
            for (y, line) in s.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    *grid.get_mut(x, y) = Tile::from_char(c)
                }
            }
            grid
        };
        let line_warp: Vec<_> = (0..grid.height)
            .map(|y| {
                let line = grid.get_line(y);
                let left = line.iter().position(|t| !t.is_empty()).unwrap();
                let right = line.iter().rposition(|t| !t.is_empty()).unwrap();
                if line[left] != Tile::Wall && line[right] != Tile::Wall {
                    (Some(((left, y), Dir::Right)), Some(((right, y), Dir::Left)))
                } else {
                    (None, None)
                }
            })
            .collect();
        let column_warp: Vec<_> = (0..grid.width)
            .map(|x| {
                let mut column = grid.get_column_iter(x);
                let (up, t1) = column.find_position(|t| !t.is_empty()).unwrap();
                let (down, t2) = column.rev().find_position(|t| !t.is_empty()).unwrap();
                if t1 != &Tile::Wall && t2 != &Tile::Wall {
                    (
                        Some(((x, up), Dir::Down)),
                        Some(((x, grid.height - down - 1), Dir::Up)),
                    )
                } else {
                    (None, None)
                }
            })
            .collect();
        let x = grid
            .get_line(0)
            .iter()
            .position(|t| t != &Tile::Empty)
            .unwrap();
        Self {
            grid,
            line_warp,
            column_warp,
            coord: (x, 0),
            dir: Dir::Right,
        }
    }
    pub fn turn(&mut self, t: Turn) {
        self.dir = self.dir.turn(t)
    }
    fn coord_forward(&self, x: usize, y: usize, dir: Dir) -> Option<(Coord, Dir)> {
        match dir {
            Dir::Left => match self.line_warp[y] {
                (_, Some(wr)) => {
                    if x == 0 || self.grid.get(x - 1, y).is_empty() {
                        Some(wr)
                    } else if self.grid.get(x - 1, y).is_wall() {
                        None
                    } else {
                        Some(((x - 1, y), dir))
                    }
                }
                (_, None) => {
                    if x == 0 || !self.grid.get(x - 1, y).is_open() {
                        None
                    } else {
                        Some(((x - 1, y), dir))
                    }
                }
            },
            Dir::Right => match self.line_warp[y] {
                (Some(wl), _) => {
                    if x == self.grid.width - 1 || self.grid.get(x + 1, y).is_empty() {
                        Some(wl)
                    } else if self.grid.get(x + 1, y).is_wall() {
                        None
                    } else {
                        Some(((x + 1, y), dir))
                    }
                }
                (None, _) => {
                    if x == self.grid.width - 1 || !self.grid.get(x + 1, y).is_open() {
                        None
                    } else {
                        Some(((x + 1, y), dir))
                    }
                }
            },
            Dir::Up => match self.column_warp[x] {
                (_, Some(wd)) => {
                    if y == 0 || self.grid.get(x, y - 1).is_empty() {
                        Some(wd)
                    } else if self.grid.get(x, y - 1).is_wall() {
                        None
                    } else {
                        Some(((x, y - 1), dir))
                    }
                }
                (_, None) => {
                    if y == 0 || !self.grid.get(x, y - 1).is_open() {
                        None
                    } else {
                        Some(((x, y - 1), dir))
                    }
                }
            },
            Dir::Down => match self.column_warp[x] {
                (Some(wu), _) => {
                    if y == self.grid.height - 1 || self.grid.get(x, y + 1).is_empty() {
                        Some(wu)
                    } else if self.grid.get(x, y + 1).is_wall() {
                        None
                    } else {
                        Some(((x, y + 1), dir))
                    }
                }
                (None, _) => {
                    if y == self.grid.height - 1 || !self.grid.get(x, y + 1).is_open() {
                        None
                    } else {
                        Some(((x, y + 1), dir))
                    }
                }
            },
        }
    }
    pub fn forward(&mut self, n: i32) {
        for _ in 0..n {
            if let Some(c) = self.coord_forward(self.coord.0, self.coord.1, self.dir) {
                // if self.coord.0.abs_diff(c.0.0) > 2 || self.coord.1.abs_diff(c.0.1) > 2 {
                //     println!("{:?} => {:?} {:?}", self.coord, c, self.dir);
                // }
                assert_eq!(self.grid.get(c.0 .0, c.0 .1), &Tile::Open);
                self.coord = c.0;
                self.dir = c.1;
            } else {
                break;
            }
        }
    }
}

enum Alternate {
    Turn,
    Forward,
}

impl Alternate {
    pub fn switch(&self) -> Self {
        match self {
            Self::Turn => Self::Forward,
            Self::Forward => Self::Turn,
        }
    }
}

struct Path {
    turns: Vec<Turn>,
    forwards: Vec<i32>,
    alternate: Alternate,
}

impl Path {
    pub fn from_str(s: &str) -> Self {
        let turns = s
            .split(|c: char| c.is_ascii_digit())
            .filter(|&s| s == "L" || s == "R")
            .map(|s| Turn::from_char(s.chars().next().unwrap()))
            .rev()
            .collect();
        let forwards = s
            .split(|c| c == 'L' || c == 'R')
            .map(|s| s.trim().parse().unwrap())
            .rev()
            .collect();
        Self {
            turns,
            forwards,
            alternate: Alternate::Forward,
        }
    }
    pub fn next(&mut self) -> Option<Either<Turn, i32>> {
        let res = match self.alternate {
            Alternate::Turn => Either::Left(self.turns.pop()),
            Alternate::Forward => Either::Right(self.forwards.pop()),
        };
        self.alternate = self.alternate.switch();
        res.factor_none()
    }
}

fn parse_input(input: &str) -> (Board, Path) {
    let (board_input, path_input) = input.split_once("\n\n").unwrap();

    (Board::from_str(board_input), Path::from_str(path_input))
}

pub fn day22_1(input: &str) -> usize {
    let (mut board, mut path) = parse_input(input);
    // dbg!(&board.line_warp, &board.column_warp);
    while let Some(command) = path.next() {
        match command {
            Either::Left(t) => board.turn(t),
            Either::Right(f) => board.forward(f),
        }
    }
    (board.coord.1 + 1) * 1000
        + (board.coord.0 + 1) * 4
        + match board.dir {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
}

pub fn day22_2(input: &str) -> usize {
    let (mut board, mut path) = parse_input(input);
    board.column_warp = custom_column_warp(&board.grid);
    board.line_warp = custom_line_warp(&board.grid);

    while let Some(command) = path.next() {
        match command {
            Either::Left(t) => board.turn(t),
            Either::Right(f) => board.forward(f),
        };
    }
    (board.coord.1 + 1) * 1000
        + (board.coord.0 + 1) * 4
        + match board.dir {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
}
// 155222 high
// 40362 low
// 64238 low

const _TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

fn warp_leads_to_wall(grid: &Grid<Tile>, warp: Warp) -> bool {
    grid.get(warp.0 .0, warp.0 .1) == &Tile::Wall
}

fn schlong_warp(grid: &Grid<Tile>, warp: Warp) -> Option<Warp> {
    debug_assert_ne!(grid.get(warp.0 .0, warp.0 .1), &Tile::Empty);
    if warp_leads_to_wall(grid, warp) {
        None
    } else {
        Some(warp)
    }
}

fn schlong_warp_list(grid: &Grid<Tile>, warp_list: &mut [(Option<Warp>, Option<Warp>)]) {
    for warp in warp_list.iter_mut() {
        *warp = (
            warp.0.and_then(|w| schlong_warp(grid, w)),
            warp.1.and_then(|w| schlong_warp(grid, w)),
        );
        // if let Some((warp_left, warp_right)) = warp {

        //     *warp = if let (Some(w1), Some(w2)) = (
        //         schlong_warp(grid, *warp_left),
        //         schlong_warp(grid, *warp_right),
        //     ) {
        //         Some((w1, w2))
        //     } else {
        //         None
        //     }
        // }
    }
}

fn custom_line_warp(grid: &Grid<Tile>) -> Vec<(Option<Warp>, Option<Warp>)> {
    let mut custom = Vec::with_capacity(200);
    for i in 0..50 {
        custom.push((
            Some(((99, 149 - i), Dir::Left)),
            Some(((0, 149 - i), Dir::Right)),
        ))
    }
    for i in 0..50 {
        custom.push((Some(((100 + i, 49), Dir::Up)), Some(((i, 100), Dir::Down))))
    }
    for i in 0..50 {
        custom.push((
            Some(((149, 49 - i), Dir::Left)),
            Some(((50, 49 - i), Dir::Right)),
        ))
    }
    for i in 0..50 {
        custom.push((
            Some(((50 + i, 149), Dir::Up)),
            Some(((50 + i, 0), Dir::Down)),
        ))
    }
    schlong_warp_list(grid, &mut custom);
    custom
}

fn custom_column_warp(grid: &Grid<Tile>) -> Vec<(Option<Warp>, Option<Warp>)> {
    let mut custom = Vec::with_capacity(150);
    for i in 0..50 {
        custom.push((
            Some(((100 + i, 0), Dir::Down)),
            Some(((50, 50 + i), Dir::Right)),
        ))
    }
    for i in 0..50 {
        custom.push((
            Some(((49, 150 + i), Dir::Left)),
            Some(((0, 150 + i), Dir::Right)),
        ))
    }
    for i in 0..50 {
        custom.push((Some(((99, 50 + i), Dir::Left)), Some(((i, 199), Dir::Up))))
    }
    schlong_warp_list(grid, &mut custom);
    custom
}
