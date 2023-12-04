#![allow(dead_code)]

use itertools::Itertools;

fn priority(c: char) -> Option<u8> {
    match c {
        'a'..='z' => Some(c as u8 - b'a' + 1),
        'A'..='Z' => Some(c as u8 - b'A' + 27),
        _ => None,
    }
}

pub fn day3_1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (frst, scnd) = line.split_at(line.len() / 2);
            // let map = std::collections::HashSet::<char>::from_iter(frst.chars());
            scnd.chars().find(|c| frst.chars().contains(c))
        })
        .filter_map(priority)
        .fold(0, |acc, n| acc + n as i32)
}

pub fn day3_2(input: &str) -> i32 {
    input
        .lines()
        .tuples()
        .filter_map(|(frst, scnd, thrd)| {
            // let frst_map = std::collections::HashSet::<char>::from_iter(frst.chars());
            // let scnd_iter = scnd.chars().filter(|c| frst_map.contains(c));
            // let scnd_map = std::collections::HashSet::<char>::from_iter(scnd_iter);
            thrd.chars()
                .find(|c| scnd.chars().contains(c) && frst.chars().contains(c))
        })
        .filter_map(priority)
        .fold(0, |acc, n| acc + n as i32)
}
