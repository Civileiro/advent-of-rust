#![allow(dead_code)]

use itertools::Itertools;

fn range_str_to_tuple(range: &str) -> Option<(i32, i32)> {
    range
        .split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples()
        .next()
}

fn line_to_range_tuple(line: &str) -> Option<((i32, i32), (i32, i32))> {
    line.split(',')
        .tuples()
        .find_map(|(r1, r2)| Some((range_str_to_tuple(r1)?, range_str_to_tuple(r2)?)))
}

fn overlaps((x1, x2): (i32, i32), (y1, y2): (i32, i32)) -> bool {
    x1 <= y2 && y1 <= x2
}

fn completely_overlaps((x1, x2): (i32, i32), (y1, y2): (i32, i32)) -> bool {
    y1 <= x1 && x2 <= y2 || x1 <= y1 && y2 <= x2
}

pub fn day4_1(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(line_to_range_tuple)
        .filter(|&(r1, r2)| completely_overlaps(r1, r2))
        .count()
}

pub fn day4_2(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(line_to_range_tuple)
        .filter(|&(r1, r2)| overlaps(r1, r2))
        .count()
}
