use crate::Day;
use crate::ParsingError;
use crate::{CountValid, Validate};
use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    min: usize,
    max: usize,
    character: char,
    val: String,
}

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

pub struct Day2 {}
impl Day<Input, usize> for Day2 {
    fn get_num(&self) -> usize {
        2
    }

    fn part1(&self, input: &Vec<Input>) -> usize {
        input.count_valid1()
    }

    fn part2(&self, input: &Vec<Input>) -> usize {
        input.count_valid2()
    }

    fn part1_answer(&self) -> usize {
        424
    }

    fn part2_answer(&self) -> usize {
        747
    }
}
