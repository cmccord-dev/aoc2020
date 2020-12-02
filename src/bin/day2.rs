use aoc20::parse_list;
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
        let a = self.val.chars().nth(self.min-1).unwrap() == self.character;
        let b = self.val.chars().nth(self.max-1).unwrap() == self.character;
        a ^ b
    }
}
impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(|x| x == '-' || x == ' ' || x == ':')
            .collect::<Vec<&str>>();
        Ok(Input {
            min: parts[0].parse().unwrap(),
            max: parts[1].parse().unwrap(),
            character: parts[2].chars().next().unwrap(),
            val: parts[4].into(),
        })
    }
}

fn part1(input: &Vec<Input>) {
    println!("Part 1: {}", input.iter().filter(|x| x.validate1()).count());
}

fn part2(input: &Vec<Input>) {
    println!("Part 2: {}", input.iter().filter(|x| x.validate2()).count());
}

fn main() {
    println!("Day 2:");
    let input: Vec<Input> = parse_list("input/day2.in");
    part1(&input);
    part2(&input);
}

