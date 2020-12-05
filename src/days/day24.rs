use std::convert::Infallible;
use crate::ParsingError;
use std::str::FromStr;
use itertools::Itertools;

use crate::DayTrait;
type Input = u64;
type Output = u64;

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        24
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        0
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        0
    }

    fn part1_answer(&self) -> Output {
        0
    }

    fn part2_answer(&self) -> Output {
        0
    }
}