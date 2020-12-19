use crate::ParsingError;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

use crate::DayTrait;
//type Input = u64;
type Output = u64;

#[derive(Debug, Clone)]
pub enum RuleType {
    Terminal(char),
    Nonterminal(Vec<Vec<usize>>),
}
#[derive(Debug, Clone)]
pub struct Rule {
    num: usize,
    rule: RuleType,
}

impl FromStr for Rule {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect_vec();
        let num = parts[0].parse()?;

        let options = parts[1].split(" | ").collect_vec();

        let rule = if options.len() == 1 && options[0].starts_with('"') {
            RuleType::Terminal(options[0].chars().skip(1).next().unwrap())
        } else {
            RuleType::Nonterminal(
                options
                    .iter()
                    .map(|&x| x.split(" ").map(|f| f.parse()).try_collect())
                    .try_collect()?,
            )
        };
        Ok(Self { num, rule })
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

impl Rule {
    fn validate<'a>(&self, s: &'a str, rules: &HashMap<usize, Rule>) -> Result<&'a str, ()> {
        match &self.rule {
            RuleType::Terminal(c) => {
                if s.starts_with(*c) {
                    Ok(&s[1..])
                } else {
                    Err(())
                }
            }
            RuleType::Nonterminal(options) => options
                .iter()
                .find_map(|option| {
                    option
                        .iter()
                        .try_fold(s, |x, y| rules[y].validate(x, rules))
                        .ok()
                })
                .ok_or(()),
        }
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
        input.iter_mut().for_each(|input| {
            //extend rules

            //reverse input
            input.input = input
                .input
                .iter()
                .map(|x| x.chars().rev().collect())
                .collect();

            //reverse rules
            input.rules.iter_mut().for_each(|x| match &mut x.1.rule {
                RuleType::Terminal(_) => (),
                RuleType::Nonterminal(options) => {
                    options.iter_mut().for_each(|y| y.reverse());
                }
            });

            input.rules.insert(
                8,
                Rule {
                    num: 8,
                    rule: RuleType::Nonterminal(vec![vec![42, 8], vec![42]]),
                },
            );
            input.rules.insert(
                11,
                Rule {
                    num: 11,
                    rule: RuleType::Nonterminal(vec![vec![31, 11, 42], vec![31, 42]]),
                },
            );
        });

        self.part1(input)
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
