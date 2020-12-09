use chrono::Datelike;
use aoc20::days::*;

fn main() {
    match chrono::Local::now().day() {
        1 => day1::Day::new().run_day(),
        2 => day2::Day::new().run_day(),
        3 => day3::Day::new().run_day(),
        4 => day4::Day::new().run_day(),
        5 => day5::Day::new().run_day(),
        6 => day6::Day::new().run_day(),
        7 => day7::Day::new().run_day(),
        8 => day8::Day::new().run_day(),
        9 => day9::Day::new().run_day(),
        10 => day10::Day::new().run_day(),
        11 => day11::Day::new().run_day(),
        12 => day12::Day::new().run_day(),
        13 => day13::Day::new().run_day(),
        14 => day14::Day::new().run_day(),
        15 => day15::Day::new().run_day(),
        16 => day16::Day::new().run_day(),
        17 => day17::Day::new().run_day(),
        18 => day18::Day::new().run_day(),
        19 => day19::Day::new().run_day(),
        20 => day20::Day::new().run_day(),
        21 => day21::Day::new().run_day(),
        22 => day22::Day::new().run_day(),
        23 => day23::Day::new().run_day(),
        24 => day24::Day::new().run_day(),
        _ => day25::Day::new().run_day(),
    };
}