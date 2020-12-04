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

pub trait Day<Input, Output>
where
    Input: FromStr + Debug,
    <Input as FromStr>::Err: Debug,
    Output: Eq + Display + Debug,
{
    //type Input: FromStr + Debug;
    fn get_num(&self) -> usize;
    fn part1_answer(&self) -> Output;
    fn part2_answer(&self) -> Output;
    fn part1(&self, input: &Vec<Input>) -> Output;
    fn part2(&self, input: &Vec<Input>) -> Output;
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list(&format!("input/day{}.in", self.get_num()))
    }
    fn run_day(&self) {
        println!("Day {}:", self.get_num());
        let input = self.read_input().unwrap();
        let b1 = Instant::now();
        let p1 = self.part1(&input);
        let b2 = Instant::now();
        println!(
            "Part1: {}\nTook {}ms",
            p1,
            (b2 - b1).as_nanos() as f64 / 1000000.0
        );

        let b1 = Instant::now();
        let p1 = self.part2(&input);
        let b2 = Instant::now();
        println!(
            "Part2: {}\nTook {}ms\n",
            p1,
            (b2 - b1).as_nanos() as f64 / 1000000.0
        );
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_day<Input, Output>(day: &dyn Day<Input, Output>)
    where
        Input: FromStr + Debug,
        <Input as FromStr>::Err: Debug,
        Output: Eq + Display + Debug,
    {
        let input = day.read_input().unwrap();
        assert_eq!(day.part1(&input), day.part1_answer());
        assert_eq!(day.part2(&input), day.part2_answer());
    }


    #[test]
    fn day1() {
        test_day(&day1::Day1 {})
    }
    #[test]
    fn day2() {
        test_day(&day2::Day2 {})
    }
    #[test]
    fn day3() {
        test_day(&day3::Day3 {})
    }
    #[test]
    fn day4() {
        test_day(&day4::Day4 {})
    }
}
