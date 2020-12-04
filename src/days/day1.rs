use itertools::Itertools;
use std::str::FromStr;

use crate::Day;

fn find_sum_to(input: &Vec<u64>, num_vals: usize) -> u64 {
    input
        .into_iter()
        .combinations(num_vals)
        .find(|x| x.iter().map(|&x| *x).sum::<u64>() == 2020u64)
        .unwrap()
        .iter()
        .map(|&x| x)
        .product::<u64>()
}
pub struct Day1 {}
impl Day<u64, u64> for Day1 {
    fn get_num(&self) -> usize {
        1
    }

    fn part1(&self, input: &Vec<u64>) -> u64 {
        find_sum_to(input, 2)
    }

    fn part2(&self, input: &Vec<u64>) -> u64 {
        find_sum_to(input, 3)
    }

    fn part1_answer(&self) -> u64 {
        1014624
    }

    fn part2_answer(&self) -> u64 {
        80072256
    }
}
