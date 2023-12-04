// extern crate adventofrust2022;

use std::time::Duration;

use adventofrust2022::time_all_days_2022;
use adventofrust2023::time_all_days_2023;

fn main() {
    let total: Duration = [
        time_all_days_2022("2022/inputs/"),
        time_all_days_2023("2023/inputs/"),
    ]
    .into_iter()
    .sum();

    println!("[{:>6}ms] final total", total.as_millis());
}
