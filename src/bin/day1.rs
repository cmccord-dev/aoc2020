use std::str::FromStr;

use aoc20::parse_list;

type Input = u64;

fn part1(input: &Vec<Input>) -> u64 {
    let mut val = 0;
    input
        .iter()
        .find(|&x| match input.iter().find(|&y| x + y == 2020) {
            None => false,
            Some(y) => {
                //println!("Part 1: {}*{} = {}", x, y, x * y);
                val = x * y;
                true
            }
        });
    val
}

fn part2(input: &Vec<Input>) -> u64 {
    let mut val = 0;
    input.iter().find(|&x| {
        input
            .iter()
            .find(|&y| match input.iter().find(|&z| x + y + z == 2020) {
                None => false,
                Some(z) => {
                    //println!("Part 2: {}*{}*{} = {}", x, y, z, x * y * z);
                    val = x * y * z;
                    true
                }
            })
            .map(|_| true)
            .unwrap_or(false)
    });
    val
}

fn read_input() -> Result<Vec<Input>, <Input as FromStr>::Err> {
    parse_list("input/day1.in")
}
fn main() {
    println!("Day 1:");
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
        assert_eq!(part1(&input), 1014624);
        assert_eq!(part2(&input), 80072256);
    }
}
