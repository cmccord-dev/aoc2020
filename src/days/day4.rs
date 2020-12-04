use crate::Day;
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

use crate::{CountValid, ParsingError, Validate, parse_list_delim, regex_match};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum PassportField {
    BirthYear,
    IssueYear,
    ExpirYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}
impl FromStr for PassportField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "byr" => PassportField::BirthYear,
            "iyr" => PassportField::IssueYear,
            "eyr" => PassportField::ExpirYear,
            "hgt" => PassportField::Height,
            "hcl" => PassportField::HairColor,
            "ecl" => PassportField::EyeColor,
            "pid" => PassportField::PassportId,
            "cid" => PassportField::CountryId,
            _ => return Err(format!("Invalid Passport Field: {}", s)),
        })
    }
}
#[derive(Debug)]
pub struct PassportItem {
    key: PassportField,
    value: String,
}
impl FromStr for PassportItem {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        Ok(PassportItem {
            key: parts[0].parse()?,
            value: parts[1].parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Input(Vec<PassportItem>);

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.split_ascii_whitespace()
                .map(|x| x.parse::<PassportItem>())
                .try_collect()?,
        ))
    }
}

impl Validate for PassportItem {
    fn validate2(&self) -> bool {
        match self.key {
            PassportField::BirthYear => match self.value.parse() {
                Ok(1920..=2002) => true,
                _ => false,
            },
            PassportField::IssueYear => match self.value.parse() {
                Ok(2010..=2020) => true,
                _ => false,
            },
            PassportField::ExpirYear => match self.value.parse() {
                Ok(2020..=2030) => true,
                _ => false,
            },
            PassportField::Height => {
                if self.value.ends_with("cm") {
                    match self.value[0..self.value.len() - 2].parse() {
                        Ok(150..=193) => true,
                        _ => false,
                    }
                } else if self.value.ends_with("in") {
                    match self.value[0..self.value.len() - 2].parse() {
                        Ok(59..=76) => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            PassportField::HairColor => {
                let mut iter = self.value.chars();
                if iter.next().unwrap() != '#' && self.value.len() != 7 {
                    false
                } else {
                    iter.all(|x| x.is_ascii_hexdigit())
                }
            }
            PassportField::EyeColor => match self.value.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            PassportField::PassportId => {
                //regex_match(r"^\d{9}$", &self.value)
                if self.value.len() != 9 {
                    false
                } else {
                    self.value.chars().all(|x| x.is_ascii_digit())
                }
            }
            PassportField::CountryId => true,
        }
    }
}

impl Validate for Input {
    fn validate1(&self) -> bool {
        let s: HashSet<PassportField> = (self.0.iter().map(|p| p.key)).collect();
        match s.len() {
            8 => true,
            7 => !s.contains(&PassportField::CountryId),
            _ => false,
        }
    }

    fn validate2(&self) -> bool {
        self.validate1() && self.0.iter().all(|v| v.validate2())
    }
}


pub struct Day4{}
impl Day<Input, usize> for Day4 {
    fn get_num(&self) -> usize {
        4
    }

    fn part1_answer(&self) -> usize {
        264
    }

    fn part2_answer(&self) -> usize {
        224
    }

    fn part1(&self, input: &Vec<Input>) -> usize {
        input.count_valid1()
    }
    
    fn part2(&self, input: &Vec<Input>) -> usize {
        input.count_valid2()
    }
    
    fn read_input(&self, ) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list_delim::<Input>(&format!("input/day{}.in", self.get_num()), "\n\n")
    }
}