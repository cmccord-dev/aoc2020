use std::convert::Infallible;
use std::str::FromStr;
use itertools::Itertools;

use crate::DayTrait;

type Input = BoardingPass;
type Output = u64;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct BoardingPass(u64);

impl FromStr for BoardingPass {
    type Err = Infallible;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        Ok(BoardingPass (
             s.chars().map(|x| if x == 'B' || x == 'R' { 1 } else { 0 }).fold(0, |p,c| (p<<1)|c)
        ))
     }
}
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        5
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input.iter().map(|x| x.0).max().unwrap()
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        input.sort();
        input.windows(2).find(|x| x[1].0-x[0].0 == 2).unwrap()[0].0+1
    }

    fn part1_answer(&self) -> Output {
        806
    }

    fn part2_answer(&self) -> Output {
        562
    }
}
