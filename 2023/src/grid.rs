pub trait Grid {
    type Item;

    fn x0(&self) -> usize;
    fn y0(&self) -> usize;
    fn x1(&self) -> usize;
    fn y1(&self) -> usize;
    fn index(&self, x: usize, y: usize) -> Option<Self::Item>;
    fn width(&self) -> usize {
        self.x1() - self.x0() + 1
    }
    fn height(&self) -> usize {
        self.y1() - self.y0() + 1
    }
    fn coord_iter(&self) -> CoordIterator {
        CoordIterator::new(self.x0(), self.y0(), self.x1(), self.y1())
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
    pub fn range(&self, line: usize, x0: usize, x1: usize) -> &[u8] {
        let line_gap = line * (self.width + 1);
        &self.ascii[(line_gap + x0)..(line_gap + x1)]
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

    fn index(&self, x: usize, y: usize) -> Option<Self::Item> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.ascii[y * (self.width + 1) + x])
        }
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
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.y1 {
            return None;
        }
        let res = Some((self.x, self.y));
        if self.x >= self.x1 {
            self.x = self.x0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        res
    }
}
