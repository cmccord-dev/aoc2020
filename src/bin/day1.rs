use aoc20::parse_list;

fn part1(input: &Vec<u64>) {
    input
        .iter()
        .find(|&x| match input.iter().find(|&y| x + y == 2020) {
            None => false,
            Some(y) => {
                println!("Part 1: {}*{} = {}", x, y, x * y);
                true
            }
        });
}

fn part2(input: &Vec<u64>) {
    input.iter().find(|&x| {
        input
            .iter()
            .find(|&y| match input.iter().find(|&z| x + y + z == 2020) {
                None => false,
                Some(z) => {
                    println!("Part 2: {}*{}*{} = {}", x, y, z, x * y * z);
                    true
                }
            })
            .map(|_| true)
            .unwrap_or(false)
    });
}

fn main() {
    println!("Day 1:");
    let input: Vec<u64> = parse_list("input/day1.in");
    part1(&input);
    part2(&input);
}
