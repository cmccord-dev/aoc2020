use itertools::Itertools;

use crate::DayTrait;
type Input = u64;
type Output = u64;

const NUM: usize = 25;
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        9
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input.windows(NUM+1).find(|x| {
            let val = x[NUM];
            match x[..NUM].iter().tuple_combinations().find(|(&y,&z)| y+z==val) {
                Some(_) => false,
                None => true
            }
        }).unwrap()[NUM]
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let target = self.part1(input.clone());
        let mut start = 0;
        let mut end = 1;
        let mut sum = input[0]+input[1];
        loop {
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => {
                    end += 1;
                    sum += input[end];
                }
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Greater => {
                    sum -= input[start];
                    start += 1;
                }
            }
        }

        match input[start..=end].iter().minmax() {
            itertools::MinMaxResult::NoElements => 0,
            itertools::MinMaxResult::OneElement(x) => x+x,
            itertools::MinMaxResult::MinMax(x, y) => x+y
        }
    }

    fn part1_answer(&self) -> Output {
        3199139634
    }

    fn part2_answer(&self) -> Output {
        438559930
    }
}