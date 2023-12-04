#![allow(dead_code)]

pub fn day20_1(input: &str) -> i32 {
    let numbers: Vec<i16> = input.lines().map(|l| l.parse::<i16>().unwrap()).collect();

    let mut indices: Vec<_> = (0..numbers.len()).map(|n| n as i16).collect();

    for (original_i, n) in numbers.iter().copied().enumerate() {
        let i = indices
            .iter()
            .position(|i| *i as usize == original_i)
            .unwrap();
        let new_i = (i as i16 + n).rem_euclid(indices.len() as i16 - 1) as usize;
        if i < new_i {
            indices[i..=new_i].rotate_left(1)
        } else {
            indices[new_i..=i].rotate_right(1)
        }
    }

    let mixed: Vec<_> = indices.into_iter().map(|i| numbers[i as usize]).collect();
    let zero = mixed.iter().position(|n| *n == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|m| mixed[(zero + m) % numbers.len()] as i32)
        .sum()
}

pub fn day20_2(input: &str) -> i64 {
    let orig_numbers: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect();

    let numbers: Vec<_> = orig_numbers
        .iter()
        .copied()
        .map(|n| n.rem_euclid(orig_numbers.len() as i64 - 1))
        .collect();

    let mut indices: Vec<_> = (0..numbers.len()).map(|n| n as i16).collect();

    for _ in 0..10 {
        for (original_i, n) in numbers.iter().copied().enumerate() {
            let i = indices
                .iter()
                .position(|i| *i as usize == original_i)
                .unwrap();

            let new_i = i + n as usize;
            let new_i = if new_i >= numbers.len() - 1 {
                new_i - (numbers.len() - 1)
            } else {
                new_i
            };
            if i < new_i {
                indices[i..=new_i].rotate_left(1)
            } else {
                indices[new_i..=i].rotate_right(1)
            }
        }
    }
    let mixed: Vec<_> = indices
        .into_iter()
        .map(|i| orig_numbers[i as usize])
        .collect();
    let zero_pos = mixed.iter().position(|n| *n == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|m| mixed[(zero_pos + m) % numbers.len()])
        .sum()
}

const _TEST_INPUT: &str = "1
2
-3
3
-2
0
4
";
