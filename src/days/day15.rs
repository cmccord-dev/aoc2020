use crate::ParsingError;
use itertools::Itertools;
use std::str::FromStr;
use std::{collections::HashMap, convert::Infallible};

use crate::DayTrait;
type Input = u64;
type Output = u64;

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        15
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut map: HashMap<u64, Vec<u64>> = input
            .iter()
            .enumerate()
            .map(|x| (*x.1, vec![x.0 as Input+1]))
            .collect();
        let mut turn = input.len() as u64;
        let mut last = *input.last().unwrap();
        while turn < 2020 {
            turn += 1;
            let last_turns = &map[&last];
            let diff = if last_turns.len() < 2 {
                0u64
            } else {
                last_turns[last_turns.len() - 1] - last_turns[last_turns.len() - 2]
            };
            //dbg!(&last_turns, &last, &diff, &turn);
            if !map.contains_key(&diff) {
                map.insert(diff, Vec::new());
            }
            map.get_mut(&diff).unwrap().push(turn);
            last = diff;

            //dbg!(&turn, &last);
        }
        last
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        
        let mut map: HashMap<u64, Vec<u64>> = input
            .iter()
            .enumerate()
            .map(|x| (*x.1, vec![x.0 as Input+1]))
            .collect();
        let mut turn = input.len() as u64;
        let mut last = *input.last().unwrap();
        while turn < 30000000 {
            turn += 1;
            let last_turns = &map[&last];
            let diff = if last_turns.len() < 2 {
                0u64
            } else {
                last_turns[last_turns.len() - 1] - last_turns[last_turns.len() - 2]
            };
            //dbg!(&last_turns, &last, &diff, &turn);
            if !map.contains_key(&diff) {
                map.insert(diff, Vec::new());
            }
            map.get_mut(&diff).unwrap().push(turn);
            last = diff;

            //dbg!(&turn, &last);
        }
        last
    }

    fn part1_answer(&self) -> Output {
        1665
    }

    fn part2_answer(&self) -> Output {
        0
    }

    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), ",")
    }
}
