use crate::DayTrait;
use itertools::Itertools;
use std::collections::HashMap;
use std::{collections::HashSet, str::FromStr};

use crate::{parse_list_delim, CountValid, ParsingError, Validate};

type Output = usize;

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
#[derive(Debug, Clone)]
pub struct Input(HashMap<PassportField, String>);

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.split_ascii_whitespace()
                .map(|x| -> Result<(PassportField, String), Self::Err> {
                    let parts: Vec<&str> = x.split(':').collect();
                    Ok((parts[0].parse()?, parts[1].parse()?))
                })
                .try_collect()?,
        ))
    }
}

impl Validate for (&PassportField, &String) {
    fn validate2(&self) -> bool {
        match self.0 {
            PassportField::BirthYear => match self.1.parse() {
                Ok(1920..=2002) => true,
                _ => false,
            },
            PassportField::IssueYear => match self.1.parse() {
                Ok(2010..=2020) => true,
                _ => false,
            },
            PassportField::ExpirYear => match self.1.parse() {
                Ok(2020..=2030) => true,
                _ => false,
            },
            PassportField::Height => {
                if self.1.ends_with("cm") {
                    match self.1[0..self.1.len() - 2].parse() {
                        Ok(150..=193) => true,
                        _ => false,
                    }
                } else if self.1.ends_with("in") {
                    match self.1[0..self.1.len() - 2].parse() {
                        Ok(59..=76) => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            PassportField::HairColor => {
                let mut iter = self.1.chars();
                if iter.next().unwrap() != '#' && self.1.len() != 7 {
                    false
                } else {
                    iter.all(|x| x.is_ascii_hexdigit())
                }
            }
            PassportField::EyeColor => match self.1.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            PassportField::PassportId => {
                //regex_match(r"^\d{9}$", &self.1)
                if self.1.len() != 9 {
                    false
                } else {
                    self.1.chars().all(|x| x.is_ascii_digit())
                }
            }
            PassportField::CountryId => true,
        }
    }
}

impl Validate for Input {
    fn validate1(&self) -> bool {
        match self.0.len() {
            8 => true,
            7 => !self.0.contains_key(&PassportField::CountryId),
            _ => false,
        }
    }

    fn validate2(&self) -> bool {
        self.validate1() && self.0.iter().all(|v| v.validate2())
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        4
    }

    fn part1_answer(&self) -> Output {
        264
    }

    fn part2_answer(&self) -> Output {
        224
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input.count_valid1()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        input.count_valid2()
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        parse_list_delim::<Input>(&format!("input/day{}.in", self.get_num()), "\n\n")
    }
}
