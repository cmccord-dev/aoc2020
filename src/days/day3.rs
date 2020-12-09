use std::ops::Deref;
use std::{convert::Infallible, str::FromStr};


use crate::{DayTrait, input_struct};
type Output = u64;


input_struct!(Input, Vec<bool>);

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.chars().map(|x| x == '.').collect()))
    }
}


fn slope(input: &Vec<Input>, down: usize, right: usize) -> u64 {
    input
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, t)| if !t[(right * i) % t.len()] { 1 } else { 0 })
        .sum()
}

#[derive(Default)]
pub struct Day{}
impl DayTrait<Input, Output> for Day{
    fn get_num(&self) -> usize {
        3
    }

    fn part1_answer(&self) -> Output {
        211
    }

    fn part2_answer(&self) -> Output {
        3584591857
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        slope(&input, 1, 3)
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|x| slope(&input, x.1, x.0))
            .product()
    }
}
