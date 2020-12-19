use crate::DayTrait;
type Input = u64;
type Output = u64;

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        10
    }

    fn part1(&self, mut input: Vec<Input>) -> Output {
        input.sort();
        let mut jolt_diff = [0, 0, 1];
        jolt_diff[(input[0] - 1) as usize] += 1;
        input
            .windows(2)
            .map(|x| x[1] - x[0] - 1)
            .for_each(|x| jolt_diff[x as usize] += 1);
        jolt_diff[0] * jolt_diff[2]
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        input.sort();
        let len = input[input.len() - 1] as usize + 3;
        let mut ways = vec![0u64; len];

        ways[len - 1] = 1;
        input.iter().rev().for_each(|&x| {
            ways[x as usize - 1] = ways[x as usize] + ways[x as usize + 1] + ways[x as usize + 2];
        });
        ways[0] + ways[1] + ways[2]
    }

    fn part1_answer(&self) -> Output {
        2232
    }

    fn part2_answer(&self) -> Output {
        173625106649344
    }
}
