use crate::ParsingError;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

use crate::DayTrait;
//type Input = u64;
type Output = u64;

#[derive(Debug, Clone)]
pub struct Rule {
    num: usize,
    options: Vec<Vec<usize>>,
    terminal: Option<char>,
}

impl FromStr for Rule {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect_vec();
        let num = parts[0].parse()?;

        let options = parts[1].split(" | ").collect_vec();

        let terminal = if options.len() == 1 && options[0].starts_with('"') {
            Some(options[0].chars().skip(1).next().unwrap())
        } else {
            None
        };
        let ops = if let None = terminal {
            options
                .iter()
                .map(|&x| x.split(" ").map(|f| f.parse()).try_collect())
                .try_collect()?
        } else {
            Vec::new()
        };
        Ok(Self {
            num,
            terminal,
            options: ops,
        })
    }
}

impl Rule {
    fn validate<'a>(&self, s: &'a str, rules: &HashMap<usize, Rule>) -> Result<&'a str, ()> {
        match self.terminal {
            Some(c) => {
                if s.starts_with(c) {
                    Ok(&s[1..])
                } else {
                    Err(())
                }
            }
            None => {
                let mut iter = self.options.iter();
                loop {
                    if let Some(option) = iter.next() {
                        match option
                            .iter()
                            .try_fold(s, |x, y| rules[y].validate(x, rules))
                        {
                            Ok(t) => {
                                break Ok(t);
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    } else {
                        break Err(());
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashMap<usize, Rule>,
    input: Vec<String>,
}

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect_vec();
        Ok(Self {
            rules: parts[0]
                .split("\n")
                .map(|x| x.parse::<Rule>().unwrap())
                .map(|x| (x.num, x))
                .collect(),
            input: parts[1].split("\n").map(|x| x.into()).collect_vec(),
        })
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        19
    }

    fn part1(&self, mut input: Vec<Input>) -> Output {
        let input = input.pop().unwrap();
        input
            .input
            .iter()
            .filter(|x| {
                input.rules[&0]
                    .validate(x.as_str(), &input.rules)
                    .map(|x| x.len() == 0)
                    .unwrap_or(false)
            })
            .count() as u64
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        let mut input = input.pop().unwrap();
        input.rules.get_mut(&8).unwrap().options.push(vec![8, 42]);
        input.rules.get_mut(&11).unwrap().options.push(vec![42, 11, 31]);
        input.input = input.input.into_iter().map(|x| x.chars().rev().collect()).collect();
        input.rules.iter_mut().for_each(|x| {
            x.1.options.iter_mut().for_each(|y| y.reverse());
            x.1.options.reverse();
        });
        self.part1(vec![input])
    }

    fn part1_answer(&self) -> Output {
        250
    }

    fn part2_answer(&self) -> Output {
        359
    }
    fn read_input(&self) -> Result<Vec<Input>, <Input as FromStr>::Err> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), "-")
    }
}
