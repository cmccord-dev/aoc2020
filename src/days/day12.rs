use crate::ParsingError;
use nalgebra::Vector2;
use std::convert::Infallible;
use std::str::FromStr;

use crate::DayTrait;
type Input = Action;
type Output = i64;

type Vec2 = Vector2<i64>;

#[derive(Debug, Clone)]
pub enum Dir {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
impl FromStr for Dir {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Self::North,
            "S" => Self::South,
            "E" => Self::East,
            "W" => Self::West,
            "L" => Self::Left,
            "R" => Self::Right,
            "F" | _ => Self::Forward,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    dir: Dir,
    amount: i64,
}

impl FromStr for Action {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            dir: s[0..1].parse()?,
            amount: s[1..].parse()?,
        })
    }
}

impl Action {
    fn go(&self, heading: Vec2, pos: Vec2) -> (Vec2, Vec2) {
        match self.dir {
            Dir::North => (heading, pos + self.amount * Vec2::new(0, 1)),
            Dir::South => (heading, pos + self.amount * Vec2::new(0, -1)),
            Dir::East => (heading, pos + self.amount * Vec2::new(1, 0)),
            Dir::West => (heading, pos + self.amount * Vec2::new(-1, 0)),
            Dir::Left => match &self.amount {
                90 => (Vec2::new(-heading[1], heading[0]), pos),
                180 => (-heading, pos),
                270 => (Vec2::new(heading[1], -heading[0]), pos),
                _ => panic!(),
            },
            Dir::Right => match &self.amount {
                90 => (Vec2::new(heading[1], -heading[0]), pos),
                180 => (-heading, pos),
                270 => (Vec2::new(-heading[1], heading[0]), pos),
                _ => panic!(),
            },
            Dir::Forward => (heading, pos + heading * self.amount),
        }
    }
    fn go2(&self, pos: Vec2, waypoint: Vec2) -> (Vec2, Vec2) {
        let waypoint_off = waypoint - pos;
        match self.dir {
            Dir::North => (pos, waypoint + self.amount * Vec2::new(0, 1)),
            Dir::South => (pos, waypoint + self.amount * Vec2::new(0, -1)),
            Dir::East => (pos, waypoint + self.amount * Vec2::new(1, 0)),
            Dir::West => (pos, waypoint + self.amount * Vec2::new(-1, 0)),
            Dir::Left => match &self.amount {
                90 => (pos, pos + Vec2::new(-waypoint_off[1], waypoint_off[0])),
                180 => (pos, pos - waypoint_off),
                270 => (pos, pos + Vec2::new(waypoint_off[1], -waypoint_off[0])),
                _ => panic!(),
            },
            Dir::Right => match &self.amount {
                270 => (pos, pos + Vec2::new(-waypoint_off[1], waypoint_off[0])),
                180 => (pos, pos - waypoint_off),
                90 => (pos, pos + Vec2::new(waypoint_off[1], -waypoint_off[0])),
                _ => panic!(),
            },
            Dir::Forward => (
                pos + waypoint_off * self.amount,
                pos + waypoint_off * (self.amount + 1),
            ),
        }
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        12
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let heading: Vec2 = Vec2::new(1, 0);
        let pos: Vec2 = Vec2::new(0, 0);
        let ans = input
            .into_iter()
            .fold((heading, pos), |p, c| c.go(p.0, p.1))
            .1
            .abs()
            .sum();
        ans
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let pos: Vec2 = Vec2::new(0, 0);
        let waypoint: Vec2 = Vec2::new(10, 1);
        let ans = input
            .into_iter()
            .fold((pos, waypoint), |p, c| c.go2(p.0, p.1))
            .0
            .abs()
            .sum();
        ans
    }

    fn part1_answer(&self) -> Output {
        1956
    }

    fn part2_answer(&self) -> Output {
        126797
    }
}
