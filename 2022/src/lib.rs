use std::time::Duration;

mod grid;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

macro_rules! run_day {
    ($($file:expr),+ => $func:expr) => {{
        use std::time;
        let mut input = String::new();
        $({
            input.push_str($file);
        })*
        let input = std::fs::read_to_string(input).unwrap();
        let start = time::Instant::now();
        let result = $func(&input);
        let time = start.elapsed();
        println!(
            "[{:>6}us] 2022 {:>16}: {}",
            time.as_micros(),
            stringify!($func),
            result
        );
        time
    }};
}

pub fn time_all_days_2022(input_folder: &str) -> Duration {
    let total: Duration = [
        run_day!(input_folder, "/1.txt" => day1::day1_1),
        run_day!(input_folder, "/1.txt" => day1::day1_2),
        run_day!(input_folder, "/2.txt" => day2::day2_1),
        run_day!(input_folder, "/2.txt" => day2::day2_2),
        run_day!(input_folder, "/3.txt" => day3::day3_1),
        run_day!(input_folder, "/3.txt" => day3::day3_2),
        run_day!(input_folder, "/4.txt" => day4::day4_1),
        run_day!(input_folder, "/4.txt" => day4::day4_2),
        run_day!(input_folder, "/5.txt" => day5::day5_1),
        run_day!(input_folder, "/5.txt" => day5::day5_2),
        run_day!(input_folder, "/6.txt" => day6::day6_1),
        run_day!(input_folder, "/6.txt" => day6::day6_2),
        run_day!(input_folder, "/7.txt" => day7::day7_1),
        run_day!(input_folder, "/7.txt" => day7::day7_2),
        run_day!(input_folder, "/8.txt" => day8::day8_1),
        run_day!(input_folder, "/8.txt" => day8::day8_2),
        run_day!(input_folder, "/9.txt" => day9::day9_1),
        run_day!(input_folder, "/9.txt" => day9::day9_2),
        run_day!(input_folder, "/10.txt" => day10::day10_1),
        run_day!(input_folder, "/10.txt" => day10::day10_2),
        run_day!(input_folder, "/11.txt" => day11::day11_1),
        run_day!(input_folder, "/11.txt" => day11::day11_2),
        run_day!(input_folder, "/12.txt" => day12::day12_1),
        run_day!(input_folder, "/12.txt" => day12::day12_2),
        run_day!(input_folder, "/13.txt" => day13::day13_1),
        run_day!(input_folder, "/13.txt" => day13::day13_2),
        run_day!(input_folder, "/14.txt" => day14::day14_1),
        run_day!(input_folder, "/14.txt" => day14::day14_2),
        run_day!(input_folder, "/15.txt" => day15::day15_1),
        run_day!(input_folder, "/15.txt" => day15::day15_2),
        run_day!(input_folder, "/16.txt" => day16::day16_1),
        run_day!(input_folder, "/16.txt" => day16::day16_2),
        run_day!(input_folder, "/17.txt" => day17::day17_1),
        run_day!(input_folder, "/17.txt" => day17::day17_2),
        run_day!(input_folder, "/18.txt" => day18::day18_1),
        run_day!(input_folder, "/18.txt" => day18::day18_2),
        run_day!(input_folder, "/19.txt" => day19::day19_1),
        run_day!(input_folder, "/19.txt" => day19::day19_2),
        run_day!(input_folder, "/20.txt" => day20::day20_1),
        run_day!(input_folder, "/20.txt" => day20::day20_2),
        run_day!(input_folder, "/21.txt" => day21::day21_1),
        run_day!(input_folder, "/21.txt" => day21::day21_2),
        run_day!(input_folder, "/22.txt" => day22::day22_1),
        run_day!(input_folder, "/22.txt" => day22::day22_2),
        run_day!(input_folder, "/23.txt" => day23::day23_1),
        run_day!(input_folder, "/23.txt" => day23::day23_2),
        run_day!(input_folder, "/24.txt" => day24::day24_1),
        run_day!(input_folder, "/24.txt" => day24::day24_2),
        run_day!(input_folder, "/25.txt" => day25::day25_1),
    ]
    .into_iter()
    .sum();

    println!("[{:>6}ms] 2022 total", total.as_millis());
    total
}
