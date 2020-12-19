use crate::ParsingError;
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};
use std::{collections::HashSet, convert::Infallible};

use crate::DayTrait;
type Input = InputData;
type Output = u64;

#[derive(Debug, Clone)]
pub struct Range {
    min: u64,
    max: u64,
}
#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    ranges: Vec<Range>,
}
#[derive(Debug, Clone)]
pub struct InputData {
    fields: Vec<Field>,
    mine: Vec<u64>,
    other: Vec<Vec<u64>>,
}
impl FromStr for Range {
    type Err = ParsingError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts = s.split('-').collect_vec();
        Ok(Range {
            min: parts[0].parse()?,
            max: parts[1].parse()?,
        })
    }
}
impl FromStr for Field {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, ParsingError> {
        let parts = s.split(": ").collect_vec();
        let parts2 = parts[1]
            .split(" or ")
            .map(|x| x.parse::<Range>())
            .try_collect()?;
        Ok(Field {
            name: parts[0].parse()?,
            ranges: parts2,
        })
    }
}

impl Range {
    fn validate(&self, data: u64) -> bool {
        self.min <= data && self.max >= data
    }
}
impl Field {
    fn validate(&self, data: u64) -> bool {
        self.ranges.iter().any(|x| x.validate(data))
    }
}
impl InputData {
    fn validate(&self, data: &[u64]) -> Option<u64> {
        data.iter()
            .find(|&x| self.fields.iter().all(|y| !y.validate(*x)))
            .map(|&x| x)
    }
}
impl FromStr for InputData {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let fields = iter
            .by_ref()
            .take_while(|&x| x != "")
            .map(|x| x.parse::<Field>())
            .try_collect()?;
        iter.next();
        let mine = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse())
            .try_collect()?;
        iter.next();
        iter.next();
        let other = iter
            .map(|x| x.split(",").map(|y| y.parse()).try_collect())
            .try_collect()?;

        Ok(InputData {
            fields,
            mine,
            other,
        })
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        16
    }
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        let file = std::fs::read_to_string(&format!("input/day{}.in", self.get_num())).unwrap();
        Ok(vec![file.parse()?])
    }
    fn part1(&self, input: Vec<Input>) -> Output {
        let input = &input[0];
        input
            .other
            .iter()
            .map(|x| input.validate(x))
            .flatten()
            .sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let input = input.into_iter().next().unwrap();
        let valid = input
            .other
            .iter()
            .filter(|x| input.validate(x).is_none())
            .collect_vec();
        let mut remapped = vec![Vec::new(); input.mine.len()];
        valid
            .iter()
            .for_each(|x| x.iter().enumerate().for_each(|y| remapped[y.0].push(*y.1)));
        let fields = input.fields.clone();
        let mut possible: Vec<HashSet<usize>> = vec![HashSet::new(); fields.len()];
        fields.iter().enumerate().for_each(|(j, x)| {
            for i in 0..fields.len() {
                if remapped[i].iter().all(|y| x.validate(*y)) {
                    possible[j].insert(i);
                }
            }
        });
        let res = possible
            .iter()
            .enumerate()
            .sorted_by_key(|x| x.1.len())
            .collect_vec();
        let mut used = HashSet::new();
        let mut mapping: HashMap<String, usize> = HashMap::new();
        res.iter().for_each(|x| {
            let intersection = x.1 - &used;
            assert_eq!(intersection.len(), 1);
            let value = intersection.into_iter().next().unwrap();
            mapping.insert(fields[x.0].name.clone(), value);
            used.insert(value);
        });
        mapping
            .iter()
            .filter(|x| x.0.starts_with("departure"))
            .map(|x| input.mine[*x.1])
            .product()
    }

    fn part1_answer(&self) -> Output {
        22057
    }

    fn part2_answer(&self) -> Output {
        1093427331937
    }
}
