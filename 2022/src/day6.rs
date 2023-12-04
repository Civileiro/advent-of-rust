use itertools::Itertools;

pub fn day6_1(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .tuple_windows()
        .position(|(n1, n2, n3, n4)| [n1, n2, n3, n4].into_iter().all_unique())
        .unwrap()
        + 4
}

pub fn day6_2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|i| i.iter().all_unique())
        .unwrap()
        + 14
}
