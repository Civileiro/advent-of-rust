#![allow(dead_code)]

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Knot {
    pub x: i64,
    pub y: i64,
}

impl Knot {
    pub fn follow(&self, other: Knot) -> Knot {
        if ((other.x - 1)..=(other.x + 1)).contains(&self.x)
            && ((other.y - 1)..=(other.y + 1)).contains(&self.y)
        {
            return *self;
        }
        let x = Self::coord_follow(self.x, other.x);
        let y = Self::coord_follow(self.y, other.y);
        Knot { x, y }
    }
    fn coord_follow(coord: i64, other: i64) -> i64 {
        match coord.cmp(&other) {
            std::cmp::Ordering::Less => coord + 1,
            std::cmp::Ordering::Equal => coord,
            std::cmp::Ordering::Greater => coord - 1,
        }
    }
    pub fn follow_dir(&self, dir: Direction) -> Knot {
        match dir {
            Direction::Left => Knot {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Knot {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Knot {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Knot {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unimplemented!(),
        }
    }
}

pub fn day9_1(input: &str) -> usize {
    let mut visited = std::collections::HashSet::<Knot>::new();
    let mut head = Knot { x: 0, y: 0 };
    let mut tail = Knot { x: 0, y: 0 };
    visited.insert(tail);
    for line in input.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let dir = Direction::from_char(dir);
        let _ = chars.next();
        let qnt = chars.as_str().parse::<i32>().unwrap();

        for _ in 0..qnt {
            head = head.follow_dir(dir);
            tail = tail.follow(head);
            visited.insert(tail);
        }
    }
    visited.len()
}

pub fn day9_2(input: &str) -> usize {
    let mut visited = std::collections::HashSet::<Knot>::new();
    let mut knots = [Knot { x: 0, y: 0 }; 10];
    visited.insert(*knots.last().unwrap());
    for line in input.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let dir = Direction::from_char(dir);
        let _ = chars.next();
        let qnt = chars.as_str().parse::<i32>().unwrap();

        for _ in 0..qnt {
            knots[0] = knots[0].follow_dir(dir);
            for i in 1..knots.len() {
                let prev = knots[i - 1];
                knots[i] = knots[i].follow(prev);
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
}
