#![allow(dead_code)]

pub fn day1_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|g| g.lines().filter_map(|s| s.parse::<i32>().ok()).sum::<i32>())
        .max()
        .unwrap_or(0)
}

pub fn day1_2(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|g| g.lines().filter_map(|s| s.parse::<i32>().ok()).sum::<i32>())
        .fold([i32::MIN; 3], |mut big3, n| {
            if n <= big3[0] {
                return big3;
            }
            big3[0] = n;
            big3.sort_unstable();
            big3
        })
        .into_iter()
        .sum()
}
