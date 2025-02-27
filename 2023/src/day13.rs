use std::fmt::Debug;

use itertools::Itertools;

use crate::grid::{AsciiGrid, Grid};

fn find_reflection<I>(items: &[I]) -> usize
where
    I: Iterator + Clone + Debug,
    I::Item: Eq,
{
    'outer: for reflection_point in 1..items.len() {
        for i in reflection_point..items.len() {
            let j = reflection_point as i64 - (i - reflection_point) as i64 - 1;
            if j < 0 {
                return reflection_point;
            }
            if items[i].clone().ne(items[j as usize].clone()) {
                continue 'outer;
            }
        }
        return reflection_point;
    }
    0
}

pub fn day13_1(input: &str) -> Result<usize, ()> {
    let res = input
        .split("\n\n")
        .map(|chunk| AsciiGrid::from_ascii(chunk.as_bytes()))
        .map(|grid| {
            let v = find_reflection(&grid.columns().collect_vec());
            let h = find_reflection(&grid.lines().collect_vec());
            v + 100 * h
        })
        .sum();
    Ok(res)
}

fn find_almost_reflection<I>(items: &[I]) -> usize
where
    I: Iterator + Clone + Debug,
    I::Item: Eq,
{
    'outer: for reflection_point in 1..items.len() {
        let mut total_errors = 0;
        for i in reflection_point..items.len() {
            let j = reflection_point as i64 - (i - reflection_point) as i64 - 1;
            if j < 0 {
                break;
            }
            total_errors += items[i]
                .clone()
                .zip(items[j as usize].clone())
                .map(|(a, b)| if a == b { 0 } else { 1 })
                .sum::<i32>();
            if total_errors > 1 {
                continue 'outer;
            }
        }
        if total_errors == 1 {
            return reflection_point;
        }
    }
    0
}

pub fn day13_2(input: &str) -> Result<usize, ()> {
    let res = input
        .split("\n\n")
        .map(|chunk| AsciiGrid::from_ascii(chunk.as_bytes()))
        .map(|grid| {
            let v = find_almost_reflection(&grid.columns().collect_vec());
            let h = find_almost_reflection(&grid.lines().collect_vec());
            v + 100 * h
        })
        .sum();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{day13_1, day13_2};

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_day13_1() {
        let res = day13_1(INPUT);
        assert_eq!(res, Ok(405))
    }

    #[test]
    fn test_day13_2() {
        let res = day13_2(INPUT);

        assert_eq!(res, Ok(400))
    }
}
