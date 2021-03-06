use crate::ParsingError;
use itertools::Itertools;
use std::str::FromStr;
use crate::DayTrait;
type Input = Schedule;
type Output = i128;

#[derive(Debug, Clone)]
pub struct Schedule {
    now: i128,
    busses: Vec<Option<i128>>,
}

impl FromStr for Schedule {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let now = lines.next().unwrap().parse()?;
        let busses = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i128>().ok())
            .collect_vec();

        Ok(Schedule { now, busses })
    }
}

fn bez_pair(a: i128, b: i128) -> (i128, i128) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    (s.0, t.0)
}

fn pair(x: (i128, i128), y: (i128, i128)) -> i128 {
    let a = (x.0, y.0);
    let n = (x.1, y.1);
    let m = bez_pair(n.0, n.1);
    let mut v = (a.0 * m.1 * n.1 + a.1 * m.0 * n.0) % (n.1 * n.0);

    if v < 0 {
        v = v + n.1 * n.0
    }
    v
}
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        13
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let input = &input[0];
        let next = input
            .busses
            .iter()
            .flatten()
            .map(|&x| {
                (
                    if input.now % x == 0 {
                        input.now
                    } else {
                        ((input.now / x) + 1) * x
                    },
                    x,
                )
            })
            .min_by_key(|x| x.0)
            .unwrap();

        (next.0 - input.now) * next.1
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        input
            .pop()
            .unwrap()
            .busses
            .into_iter()
            .enumerate()
            .filter(|x| x.1.is_some())
            .map(|x| (x.0 as i128, x.1.unwrap()))
            .map(|x| (x.1 - x.0, x.1))
            .fold1(|a, b| {
                (pair(a, b), a.1 * b.1)
            })
            .unwrap()
            .0
    }

    fn part1_answer(&self) -> Output {
        2545
    }

    fn part2_answer(&self) -> Output {
        266204454441577
    }

    fn read_input(&self) -> Result<Vec<Input>, ParsingError> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), "!")
    }
}
