use aoc20::{parse_list, ParsingError};
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    character: char,
    val: String,
}

impl Input {
    pub fn validate1(&self) -> bool {
        let count = self.val.matches(self.character).count();
        self.min <= count && count <= self.max
    }
    pub fn validate2(&self) -> bool {
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

fn part1(input: &Vec<Input>) -> usize {
    input.iter().filter(|x| x.validate1()).count()
}

fn part2(input: &Vec<Input>) -> usize {
    input.iter().filter(|x| x.validate2()).count()
}

fn read_input() -> Result<Vec<Input>, <Input as FromStr>::Err> {
    parse_list("input/day2.in")
}
fn main() {
    println!("Day 2:");
    let input = read_input().unwrap();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day1() {
        let input = read_input().unwrap();
        assert_eq!(part1(&input), 424);
        assert_eq!(part2(&input), 747);
    }
}
