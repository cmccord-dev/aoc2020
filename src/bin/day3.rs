use std::{convert::Infallible, str::FromStr};

use aoc20::parse_list;

/*#[derive(Debug)]
struct Input {
    data: Vec<bool>
}*/
struct Input(Vec<bool>);
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

fn part1(input: &Vec<Input>) -> u64 {
    slope(input, 1, 3)
}

fn part2(input: &Vec<Input>) -> u64 {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|x| slope(input, x.1, x.0))
        .product()
}

fn read_input() -> Result<Vec<Input>, <Input as FromStr>::Err> {
    parse_list("input/day3.in")
}
fn main() {
    println!("Day 3:");
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
        assert_eq!(part1(&input), 211);
        assert_eq!(part2(&input), 3584591857);
    }
}
