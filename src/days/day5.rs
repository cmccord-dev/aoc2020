use std::convert::Infallible;
use crate::ParsingError;
use std::str::FromStr;
use itertools::Itertools;

use crate::Day;

/*fn find_sum_to(input: &Vec<u64>, num_vals: usize) -> u64 {
    input
        .into_iter()
        .combinations(num_vals)
        .find(|x| x.iter().map(|&x| *x).sum::<u64>() == 2020u64)
        .unwrap()
        .iter()
        .map(|&x| x)
        .product::<u64>()
}*/

#[derive(Debug)]
pub struct BoardingPass {
    seat_location: u64
}
impl FromStr for BoardingPass {
    type Err = Infallible;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        Ok(BoardingPass {
            seat_location: s.chars().map(|x| if x == 'B' || x == 'R' { 1 } else { 0 }).fold(0, |p,c| (p<<1)|c)
        })
     }
}
pub struct Day5 {}
impl Day<BoardingPass, u64> for Day5 {
    fn get_num(&self) -> usize {
        5
    }

    fn part1(&self, input: &Vec<BoardingPass>) -> u64 {
        input.iter().map(|x| x.seat_location).max().unwrap()
    }

    fn part2(&self, input: &Vec<BoardingPass>) -> u64 {
        let mut sorted: Vec<u64> = input.iter().map(|x| x.seat_location).collect::<Vec<u64>>();
        sorted.sort();
        sorted.windows(2).find(|x| x[1]-x[0] == 2).unwrap()[0]+1
    }

    fn part1_answer(&self) -> u64 {
        806
    }

    fn part2_answer(&self) -> u64 {
        562
    }
}
