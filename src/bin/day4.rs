use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

use aoc20::{parse_list_delim, ParsingError};

const DAY: u32 = 4;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum PassportField {
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
struct PassportItem {
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

impl PassportItem {
    pub fn validate(&self) -> bool {
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
struct Input(Vec<PassportItem>);

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

fn part1(input: &Vec<Input>) -> u64 {
    input
        .iter()
        .map(|x| {
            let s: HashSet<PassportField> = (x.0.iter().map(|p| p.key)).collect();
            match s.len() {
                8 => 1u64,
                7 => {
                    if !s.contains(&PassportField::CountryId) {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        })
        .sum()
}

fn part2(input: &Vec<Input>) -> u64 {
    input
        .iter()
        .map(|x| {
            if x.0.iter().all(|v| v.validate()) {
                let s: HashSet<PassportField> = (x.0.iter().map(|p| p.key)).collect();
                match s.len() {
                    8 => 1,
                    7 => {
                        if !s.contains(&PassportField::CountryId) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                }
            } else {
                0
            }
        })
        .sum()
}

fn read_input() -> Result<Vec<Input>, <Input as FromStr>::Err> {
    parse_list_delim::<Input>(&format!("input/day{}.in", DAY), "\n\n")
}
fn main() {
    println!("Day {}:", DAY);
    let input = read_input().unwrap();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = read_input().unwrap();
        assert_eq!(part1(&input), 264);
        assert_eq!(part2(&input), 224);
    }
}
