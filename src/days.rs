use crate::parse_list;
use std::str::FromStr;
use std::{
    fmt::{Debug, Display},
    time::Instant,
};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

#[macro_export]
macro_rules! input_struct {
    ($name:ident, $t:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name($t);
        impl std::ops::Deref for $name {
            type Target = $t;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

pub trait DayTrait<Input, Output>: Default
where
    Input: FromStr + Debug + Clone,
    <Input as FromStr>::Err: Debug,
    Output: Eq + Display + Debug,
{
    //type Input: FromStr + Debug;
    fn get_num(&self) -> usize;
    fn part1_answer(&self) -> Output;
    fn part2_answer(&self) -> Output;
    fn part1(&self, input: Vec<Input>) -> Output;
    fn part2(&self, input: Vec<Input>) -> Output;
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list(&format!("input/day{}.in", self.get_num()))
    }
    fn run_day(&self) -> (f64, f64) {
        println!("Day {}:", self.get_num());
        let input = self.read_input().unwrap();
        let b1 = Instant::now();
        let p1 = self.part1(input.clone());
        let b2 = Instant::now();
        println!(
            "Part1: {}\nTook {}ms",
            p1,
            (b2 - b1).as_nanos() as f64 / 1000000.0
        );
        let part1_time = (b2 - b1).as_nanos() as f64 / 1e6;

        let b1 = Instant::now();
        let p1 = self.part2(input);
        let b2 = Instant::now();
        println!(
            "Part2: {}\nTook {}ms\n",
            p1,
            (b2 - b1).as_nanos() as f64 / 1000000.0
        );
        let part2_time = (b2 - b1).as_nanos() as f64 / 1e6;
        (part1_time, part2_time)
    }
    fn test(&self) {
        let input = self.read_input().unwrap();
        assert_eq!(self.part1(input.clone()), self.part1_answer());
        assert_eq!(self.part2(input), self.part2_answer());
    }
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1() {
        day1::Day::new().test()
    }
    #[test]
    fn day2() {
        day2::Day::new().test()
    }
    #[test]
    fn day3() {
        day3::Day::new().test()
    }
    #[test]
    fn day4() {
        day4::Day::new().test()
    }
    #[test]
    fn day5() {
        day5::Day::new().test()
    }
    #[test]
    fn day6() {
        day6::Day::new().test()
    }
    #[test]
    fn day7() {
        day7::Day::new().test()
    }
    #[test]
    fn day8() {
        day8::Day::new().test()
    }
    #[test]
    fn day9() {
        day9::Day::new().test()
    }
    #[test]
    fn day10() {
        day10::Day::new().test()
    }
    #[test]
    fn day11() {
        day11::Day::new().test()
    }
    #[test]
    fn day12() {
        day12::Day::new().test()
    }
    #[test]
    fn day13() {
        day13::Day::new().test()
    }
    #[test]
    fn day14() {
        day14::Day::new().test()
    }
    #[test]
    fn day15() {
        day15::Day::new().test()
    }
    #[test]
    fn day16() {
        day16::Day::new().test()
    }
    #[test]
    fn day17() {
        day17::Day::new().test()
    }
    #[test]
    fn day18() {
        day18::Day::new().test()
    }
    #[test]
    fn day19() {
        day19::Day::new().test()
    }
    #[test]
    fn day20() {
        day20::Day::new().test()
    }
    #[test]
    fn day21() {
        day21::Day::new().test()
    }
    #[test]
    fn day22() {
        day22::Day::new().test()
    }
    #[test]
    fn day23() {
        day23::Day::new().test()
    }
    #[test]
    fn day24() {
        day24::Day::new().test()
    }
    #[test]
    fn day25() {
        day25::Day::new().test()
    }
}
