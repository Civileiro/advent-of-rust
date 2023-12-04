#![allow(dead_code)]

use crate::grid::Grid;

struct Tree {
    pub height: u8,
    pub seen: bool,
}

fn parse_trees(input: &str) -> Grid<Tree> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid_raw = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| Tree {
                height: c as u8 - b'0',
                seen: false,
            })
        })
        .collect();
    Grid::from_vec(width, height, grid_raw)
}

fn mark_trees(trees: &mut Grid<Tree>) {
    for y in 0..trees.height {
        // left to right
        let mut curr_height = {
            let f = trees.get_mut(0, y);
            f.seen = true;
            f.height
        };
        for x in 1..trees.width {
            let i = trees.get_mut(x, y);
            if i.height <= curr_height {
                continue;
            }
            i.seen = true;
            curr_height = i.height;
        }

        // right to left
        let mut curr_height = {
            let f = trees.get_mut(trees.width - 1, y);
            f.seen = true;
            f.height
        };
        for x in (1..trees.width).rev() {
            let i = trees.get_mut(x, y);
            if i.height <= curr_height {
                continue;
            }
            i.seen = true;
            curr_height = i.height;
        }
    }
    for x in 0..trees.width {
        // left to right
        let mut curr_height = {
            let f = trees.get_mut(x, 0);
            f.seen = true;
            f.height
        };
        for y in 1..trees.height {
            let i = trees.get_mut(x, y);
            if i.height <= curr_height {
                continue;
            }
            i.seen = true;
            curr_height = i.height;
        }

        // right to left
        let mut curr_height = {
            let f = trees.get_mut(x, trees.height - 1);
            f.seen = true;
            f.height
        };
        for y in (1..trees.height).rev() {
            let i = trees.get_mut(x, y);
            if i.height <= curr_height {
                continue;
            }
            i.seen = true;
            curr_height = i.height;
        }
    }
}

pub fn day8_1(input: &str) -> usize {
    let mut trees = parse_trees(input);
    mark_trees(&mut trees);
    trees.grid_raw.into_iter().filter(|tree| tree.seen).count()
}

fn scenic_score(trees: &Grid<Tree>, xo: usize, yo: usize) -> usize {
    let init_height = trees.get(xo, yo).height;
    let right = 'view: {
        if xo == trees.width - 1 {
            break 'view 0;
        }
        for i in 1.. {
            if trees.get(xo + i, yo).height >= init_height || xo + i == trees.width - 1 {
                break 'view i;
            }
        }
        unimplemented!()
    };
    let left = 'view: {
        if xo == 0 {
            break 'view 0;
        }
        for i in 1.. {
            if trees.get(xo - i, yo).height >= init_height || xo - i == 0 {
                break 'view i;
            }
        }
        unimplemented!()
    };
    let down = 'view: {
        if yo == trees.height - 1 {
            break 'view 0;
        }
        for i in 1.. {
            if trees.get(xo, yo + i).height >= init_height || yo + i == trees.height - 1 {
                break 'view i;
            }
        }
        unimplemented!()
    };
    let up = 'view: {
        if yo == 0 {
            break 'view 0;
        }
        for i in 1.. {
            if trees.get(xo, yo - i).height >= init_height || yo - i == 0 {
                break 'view i;
            }
        }
        unimplemented!()
    };
    left * right * up * down
}

pub fn day8_2(input: &str) -> usize {
    let trees = parse_trees(input);
    let mut max_scenic = 0;
    for x in 0..trees.width {
        for y in 0..trees.height {
            let scenic = scenic_score(&trees, x, y);
            if scenic > max_scenic {
                max_scenic = scenic
            }
        }
    }
    max_scenic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut m = Grid::from_vec(3, 3, v);
        assert_eq!(m.get_mut(1, 1), &mut 5);
        assert_eq!(m.get_mut(1, 2), &mut 8);
        assert_eq!(m.get_mut(2, 2), &mut 9);
        assert_eq!(m.get_mut(0, 0), &mut 1);
    }
}
