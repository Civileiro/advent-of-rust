use crate::grid::{AsciiGrid, CoordIterator, Grid};

fn is_symbol(byte: u8) -> bool {
    !byte.is_ascii_digit() && byte != b'.'
}

fn number_end(grid: &impl Grid<Item = u8>, line: usize, x_start: usize) -> usize {
    let mut x_end = x_start;
    while let Some(c) = grid.index(x_end, line) {
        if !c.is_ascii_digit() {
            break;
        }
        x_end += 1;
    }
    x_end
}

pub fn day3_1(input: &str) -> Result<u32, ()> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let mut sum = 0;
    let mut skip = false;
    for (x, y) in grid.coord_iter() {
        let c = grid.index(x, y).unwrap();
        if !c.is_ascii_digit() {
            skip = false;
            continue;
        }
        if skip {
            continue;
        }
        let mut touches_symbol = false;
        let x0 = x;
        let x1 = number_end(&grid, y, x0);
        for (xs, ys) in CoordIterator::new(x0.saturating_sub(1), y.saturating_sub(1), x1, y + 1) {
            let Some(byte) = grid.index(xs, ys) else {
                continue;
            };
            if is_symbol(byte) {
                touches_symbol = true;
                break;
            }
        }
        if touches_symbol {
            let num = std::str::from_utf8(grid.range(y, x0, x1)).unwrap();
            let num: u32 = num.parse().unwrap();
            sum += num
        }
        skip = true;
    }
    Ok(sum)
}

#[derive(Debug, Default)]
struct PairSet<T: Eq> {
    set: [Option<T>; 2],
}

impl<T: Eq> PairSet<T> {
    fn new() -> Self {
        Self { set: [None, None] }
    }

    fn full(&self) -> bool {
        self.set.iter().all(|o| o.is_some())
    }

    fn add(&mut self, elem: T) {
        if self.contains(&elem) {
            return;
        }
        self.add_unchecked(elem)
    }

    fn add_unchecked(&mut self, elem: T) {
        for i in 0..self.set.len() {
            if self.set[i].is_none() {
                self.set[i] = Some(elem);
                return;
            }
        }
    }

    fn contains(&self, t: &T) -> bool {
        for elem in self.set.iter().flatten() {
            if elem == t {
                return true;
            }
        }
        false
    }

    fn clear(&mut self) {
        for elem in &mut self.set {
            *elem = None
        }
    }

    fn into_pair(mut self) -> Option<(T, T)> {
        self.set[0]
            .take()
            .and_then(|t1| self.set[1].take().map(|t2| (t1, t2)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GridNumber {
    x: usize,
    y: usize,
    n: u32,
}

fn get_grid_num(grid: &AsciiGrid, line: usize, x: usize) -> GridNumber {
    let mut x0 = x;
    let mut x1 = x;
    while let Some(c) = grid.index(x0, line) {
        if !c.is_ascii_digit() {
            x0 += 1;
            break;
        }
        if x0 == 0 {
            break;
        }
        x0 -= 1;
    }
    while let Some(c) = grid.index(x1, line) {
        if !c.is_ascii_digit() {
            break;
        }
        x1 += 1;
    }
    let range = grid.range(line, x0, x1);
    let n = std::str::from_utf8(range).unwrap().parse().unwrap();
    GridNumber { x: x0, y: line, n }
}

pub fn day3_2(input: &str) -> Result<u32, ()> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let mut sum = 0;
    for (x, y) in grid.coord_iter() {
        let c = grid.index(x, y).unwrap();
        if c != b'*' {
            continue;
        }
        let mut pair = PairSet::new();
        for (xs, ys) in CoordIterator::new(x.saturating_sub(1), y.saturating_sub(1), x + 1, y + 1) {
            let Some(b) = grid.index(xs, ys) else {
                continue;
            };
            if !b.is_ascii_digit() {
                continue;
            }
            let grid_num = get_grid_num(&grid, ys, xs);
            if !pair.contains(&grid_num) && pair.full() {
                pair.clear();
                break;
            }
            pair.add(grid_num)
        }
        if let Some((n1, n2)) = pair.into_pair() {
            sum += n1.n * n2.n
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::{day3_1, day3_2};

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    #[test]
    fn test_day3_1() {
        let res = day3_1(INPUT);
        assert_eq!(res, Ok(4361))
    }

    #[test]
    fn test_day3_2() {
        let res = day3_2(INPUT);
        assert_eq!(res, Ok(467835))
    }
}
