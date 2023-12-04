fn parse_line(line: &str) -> i64 {
    line.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5_i64.pow(i as u32)
                * match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => unimplemented!(),
                }
        })
        .sum()
}

fn to_snafu(mut n: i64) -> String {
    let max_power = ((n as f64 * 2f64 - 1f64).log(5f64) + 0.001f64)
        .ceil()
        .max(1f64) as u32;
    let mut snafu = String::with_capacity(max_power as usize + 1);
    for power in (0..max_power).rev().map(|i| 5i64.pow(i)) {
        let digit = (n + n.signum() * power / 2) / power;
        n -= digit * power;
        let c = match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unimplemented!(),
        };
        snafu.push(c)
    }
    snafu.to_string()
}

pub fn day25_1(input: &str) -> String {
    let sum: i64 = input.lines().map(parse_line).sum();
    to_snafu(sum)
}

const _TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
