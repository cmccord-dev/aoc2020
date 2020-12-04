use std::{convert::Infallible, str::FromStr};

use crate::Day;


#[derive(Debug)]
pub struct Input(Vec<bool>);
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
        .map(|(i, t)| if !t.0[(right * i) % t.0.len()] { 1 } else { 0 })
        .sum()
}

pub struct Day3{}
impl Day<Input, u64> for Day3{
    fn get_num(&self) -> usize {
        3
    }

    fn part1_answer(&self) -> u64 {
        211
    }

    fn part2_answer(&self) -> u64 {
        3584591857
    }

    fn part1(&self, input: &Vec<Input>) -> u64 {
        slope(input, 1, 3)
    }

    fn part2(&self, input: &Vec<Input>) -> u64 {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|x| slope(input, x.1, x.0))
            .product()
    }
}
