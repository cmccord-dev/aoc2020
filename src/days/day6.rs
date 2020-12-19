use crate::{input_struct, parse_list_delim};
use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

use crate::DayTrait;

input_struct!(Input, Vec<Vec<char>>);
type Output = usize;


impl FromStr for Input {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Input(input.lines().map(|x| x.chars().collect()).collect()))
    }
}


#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        6
    }
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list_delim(&format!("input/day{}.in", self.get_num()), "\n\n")
    }
    fn part1(&self, input: Vec<Input>) -> Output {
        input
            .into_iter()
            .map(|x| x.0.into_iter().flatten().unique().count())
            .sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
            input.into_iter().map(|x| {
                let mut arr = [0usize; 26];
                x.iter().for_each(|v| v.iter().for_each(|&c| arr[c as usize-'a' as usize]+= 1));
                arr.iter().filter(|&p| *p==x.len()).count()
            }).sum()
    }

    fn part1_answer(&self) -> Output {
        7110
    }

    fn part2_answer(&self) -> Output {
        3628
    }
}
