#![allow(dead_code)]

use itertools::Itertools;
use num::Integer;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Self {
                x: self.x - 1,
                ..*self
            })
        }
    }
    pub fn go_left(&mut self) -> bool {
        if self.x == 0 {
            false
        } else {
            self.x -= 1;
            true
        }
    }
    pub fn right(&self) -> Option<Self> {
        Some(Self {
            x: self.x + 1,
            ..*self
        })
    }
    pub fn go_right(&mut self) -> bool {
        self.x += 1;
        true
    }
    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                y: self.y - 1,
                ..*self
            })
        }
    }
    pub fn go_up(&mut self) -> bool {
        if self.y == 0 {
            false
        } else {
            self.y -= 1;
            true
        }
    }
    pub fn down(&self) -> Option<Self> {
        Some(Self {
            y: self.y + 1,
            ..*self
        })
    }
    pub fn go_down(&mut self) -> bool {
        self.y += 1;
        true
    }
    pub fn at_dir(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
    pub fn go_dir(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self.go_up(),
            Direction::Down => self.go_down(),
            Direction::Left => self.go_left(),
            Direction::Right => self.go_right(),
        }
    }
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub trait Grid {
    type Item;

    fn x0(&self) -> usize;
    fn y0(&self) -> usize;
    fn x1(&self) -> usize;
    fn y1(&self) -> usize;
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x <= self.x1() && y <= self.y1()
    }
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;
    fn get_coord(&self, Coord { x, y }: Coord) -> Option<&Self::Item> {
        self.get(x, y)
    }
    fn width(&self) -> usize {
        self.x1() - self.x0() + 1
    }
    fn height(&self) -> usize {
        self.y1() - self.y0() + 1
    }
    fn coord_iter(&self) -> CoordIterator {
        CoordIterator::new(self.x0(), self.y0(), self.x1(), self.y1())
    }
    fn find_coord<P>(&self, pred: P) -> Option<Coord>
    where
        P: Fn(&<Self as Grid>::Item) -> bool,
    {
        self.coord_iter()
            .find(|&coord| pred(self.get_coord(coord).unwrap()))
    }
    fn lines(&self) -> Lines<Self>
    where
        Self: Sized,
    {
        Lines::new(self)
    }
    fn columns(&self) -> Columns<Self>
    where
        Self: Sized,
    {
        Columns::new(self)
    }
}

pub trait MutGrid: Grid {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;
    fn get_coord_mut(&mut self, Coord { x, y }: Coord) -> Option<&mut Self::Item> {
        self.get_mut(x, y)
    }
}

pub struct CoordIterator {
    x0: usize,
    // y0: usize,
    x1: usize,
    y1: usize,
    x: usize,
    y: usize,
}

impl CoordIterator {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        assert!(x0 <= x1);
        assert!(y0 <= y1);
        Self {
            x0,
            // y0,
            x1,
            y1,
            x: x0,
            y: y0,
        }
    }
}

impl Iterator for CoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.y1 {
            return None;
        }
        let res = Some(Coord {
            x: self.x,
            y: self.y,
        });
        if self.x >= self.x1 {
            self.x = self.x0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        res
    }
}

pub struct LineIterator<'a, G: Grid> {
    grid: &'a G,
    line: usize,
    x: usize,
}

impl<'a, G: Grid> LineIterator<'a, G> {
    fn new(grid: &'a G, line: usize) -> LineIterator<'a, G> {
        Self { grid, line, x: 0 }
    }
}

impl<'a, G: Grid> Iterator for LineIterator<'a, G> {
    type Item = &'a G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.grid.get(self.x, self.line);
        self.x += 1;
        elem
    }
}

pub struct Lines<'a, G: Grid> {
    grid: &'a G,
    line: usize,
}

impl<'a, G: Grid> Lines<'a, G> {
    fn new(grid: &'a G) -> Lines<'a, G> {
        Self { grid, line: 0 }
    }
}

impl<'a, G: Grid> Iterator for Lines<'a, G> {
    type Item = LineIterator<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line == self.grid.height() {
            return None;
        }
        let iter = LineIterator::new(self.grid, self.line);
        self.line += 1;
        Some(iter)
    }
}

pub struct ColumnIterator<'a, G: Grid> {
    grid: &'a G,
    column: usize,
    y: usize,
}

impl<'a, G: Grid> ColumnIterator<'a, G> {
    fn new(grid: &'a G, column: usize) -> ColumnIterator<'a, G> {
        Self { grid, column, y: 0 }
    }
}

impl<'a, G: Grid> Iterator for ColumnIterator<'a, G> {
    type Item = &'a G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.grid.get(self.column, self.y);
        self.y += 1;
        elem
    }
}

pub struct Columns<'a, G: Grid> {
    grid: &'a G,
    column: usize,
}

impl<'a, G: Grid> Columns<'a, G> {
    fn new(grid: &'a G) -> Columns<'a, G> {
        Self { grid, column: 0 }
    }
}

impl<'a, G: Grid> Iterator for Columns<'a, G> {
    type Item = ColumnIterator<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column == self.grid.width() {
            return None;
        }
        let iter = ColumnIterator::new(self.grid, self.column);
        self.column += 1;
        Some(iter)
    }
}

pub struct AsciiGrid<'a> {
    ascii: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> AsciiGrid<'a> {
    pub fn from_ascii(ascii: &'a [u8]) -> AsciiGrid<'a> {
        let width = ascii.split(|&c| c == b'\n').next().unwrap().len();
        let mut height = 0;
        {
            let mut lines = ascii.split(|&c| c == b'\n').peekable();
            while let Some(line) = lines.next() {
                if lines.peek().is_some() {
                    assert!(line.len() == width);
                    height += 1
                } else if line.len() == width {
                    height += 1
                }
            }
        }
        Self {
            ascii,
            width,
            height,
        }
    }
    pub fn range(&self, line: usize, x0: usize, x1: usize) -> &[<Self as Grid>::Item] {
        let line_gap = line * (self.width + 1);
        &self.ascii[(line_gap + x0)..(line_gap + x1)]
    }
    pub fn find_coord<P>(&self, pred: P) -> Option<Coord>
    where
        P: FnMut(&<Self as Grid>::Item) -> bool,
    {
        let (pos, _) = self.ascii.iter().copied().find_position(pred)?;
        let (y, x) = pos.div_rem(&(self.width() + 1));
        Some(Coord { x, y })
    }
}

impl<'a> Grid for AsciiGrid<'a> {
    type Item = u8;

    fn x0(&self) -> usize {
        0
    }
    fn y0(&self) -> usize {
        0
    }
    fn x1(&self) -> usize {
        self.width - 1
    }
    fn y1(&self) -> usize {
        self.height - 1
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.ascii[y * (self.width + 1) + x])
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecGrid<T> {
    pub width: usize,
    pub height: usize,
    pub grid_raw: Vec<T>,
}

impl<T: Clone> VecGrid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let grid_raw = vec![default; width * height];
        assert_eq!(width * height, grid_raw.len());
        Self {
            width,
            height,
            grid_raw,
        }
    }
}

impl<T> VecGrid<T> {}

impl<T> Grid for VecGrid<T> {
    type Item = T;

    fn x0(&self) -> usize {
        0
    }

    fn y0(&self) -> usize {
        0
    }

    fn x1(&self) -> usize {
        self.width - 1
    }

    fn y1(&self) -> usize {
        self.height - 1
    }

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        if !self.in_bounds(x, y) {
            None
        } else {
            Some(&self.grid_raw[y * self.width + x])
        }
    }
}

impl<T> MutGrid for VecGrid<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        if !self.in_bounds(x, y) {
            None
        } else {
            Some(&mut self.grid_raw[y * self.width + x])
        }
    }
}
