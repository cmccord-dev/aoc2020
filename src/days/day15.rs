
use std::str::FromStr;

use crate::DayTrait;
type Input = usize;
type Output = usize;

fn get_nth(input: Vec<Input>, n: usize) -> Output {
    let mut cache: Vec<Output> = vec![0; n];
    input
        .iter()
        .enumerate()
        .for_each(|x| cache[*x.1] = x.0 as Output + 1);
    let mut turn = input.len() + 1;
    let mut last = 0;
    while turn < n {
        let next = if cache[last] == 0 {
            0usize
        } else {
            turn - cache[last]
        };
        cache[last] = turn;
        last = next;
        turn += 1;
    }

    last
}
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        15
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        get_nth(input, 2020)
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        get_nth(input, 30000000)
    }

    fn part1_answer(&self) -> Output {
        1665
    }

    fn part2_answer(&self) -> Output {
        16439
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), ",")
    }
}
