use itertools::Itertools;

use crate::DayTrait;
type Input = u64;
type Output = u64;

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
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        1
    }

    fn part1(&self, input: Vec<u64>) -> Output {
        let ans = input
            .iter()
            .tuple_combinations()
            .find(|(&x, &y)| x + y == 2020)
            .unwrap();
        ans.0 * ans.1
    }

    fn part2(&self, input: Vec<u64>) -> Output {
        let ans = input
            .iter()
            .tuple_combinations()
            .find(|(&x, &y, &z)| x + y + z == 2020)
            .unwrap();
        ans.0 * ans.1 * ans.2
    }

    fn part1_answer(&self) -> Output {
        1014624
    }

    fn part2_answer(&self) -> Output {
        80072256
    }
}
