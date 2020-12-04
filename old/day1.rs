use itertools::Itertools;
use std::str::FromStr;

use aoc20::parse_list;

const DAY: u32 = 1;
type Input = u64;

fn find_sum_to(input: &Vec<Input>, num_vals: usize) -> u64 {
    input
        .into_iter()
        .combinations(num_vals)
        .find(|x| x.iter().map(|&x| *x).sum::<u64>() == 2020u64)
        .unwrap()
        .iter()
        .map(|&x| x)
        .product::<u64>()
}

fn part1(input: &Vec<Input>) -> u64 {
    find_sum_to(input, 2)
}

fn part2(input: &Vec<Input>) -> u64 {
    find_sum_to(input, 3)
}

fn read_input() -> Result<Vec<Input>, <Input as FromStr>::Err> {
    parse_list(&format!("input/day{}.in", DAY))
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
        assert_eq!(part1(&input), 1014624);
        assert_eq!(part2(&input), 80072256);
    }
}
