use crate::DayTrait;
use crate::ParsingError;
use crate::{CountValid, Validate};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Input {
    min: usize,
    max: usize,
    character: char,
    val: String,
}
type Output = usize;

impl Validate for Input {
    fn validate1(&self) -> bool {
        let count = self.val.matches(self.character).count();
        self.min <= count && count <= self.max
    }
    fn validate2(&self) -> bool {
        let a = self.val.chars().nth(self.min - 1).unwrap() == self.character;
        let b = self.val.chars().nth(self.max - 1).unwrap() == self.character;
        a ^ b
    }
}
impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(|x| x == '-' || x == ' ' || x == ':')
            .collect::<Vec<&str>>();
        Ok(Input {
            min: parts[0].parse()?,
            max: parts[1].parse()?,
            character: parts[2].parse()?,
            val: parts[4].parse()?,
        })
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        2
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input.count_valid1()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        input.count_valid2()
    }

    fn part1_answer(&self) -> Output {
        424
    }

    fn part2_answer(&self) -> Output {
        747
    }
}
