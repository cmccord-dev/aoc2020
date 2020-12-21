use crate::ParsingError;
use std::fmt::Display;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use crate::DayTrait;
type Input = Item;
type Output = OutputStruct;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OutputStruct {
    Usize(usize),
    String(String),
}
impl Display for OutputStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputStruct::Usize(u) => write!(f, "{}", u),
            OutputStruct::String(s) => write!(f, "{}", s),
        }
    }
}
impl From<usize> for OutputStruct {
    fn from(u: usize) -> Self {
        Self::Usize(u)
    }
}
impl From<String> for OutputStruct {
    fn from(u: String) -> Self {
        Self::String(u)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Item {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ingredients = s[..s.find(" (").unwrap()]
            .split(" ")
            .map(|x| x.into())
            .collect_vec();
        let allergens = s[s.find("contains ").unwrap() + "contains ".len()..s.len() - 1]
            .split(", ")
            .map(|x| x.into())
            .collect_vec();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

fn get_dangerous_ingredients(input: &Vec<Input>) -> HashMap<String, String> {
    let mut allergens: HashMap<String, Vec<usize>> = HashMap::new();
    let mut eval_order = VecDeque::new();
    input.iter().enumerate().for_each(|(i, x)| {
        x.allergens.iter().for_each(|y| {
            allergens
                .entry(y.into())
                .or_insert_with(|| Vec::new())
                .push(i);
            if !eval_order.contains(y) {
                eval_order.push_back(y.clone());
            }
        })
    });
    let mut mapping: HashMap<String, String> = HashMap::new();
    loop {
        match eval_order.pop_front() {
            Some(next) => {
                let mut all = allergens[&next]
                    .iter()
                    .map(|x| {
                        input[*x]
                            .ingredients
                            .iter()
                            .map(|x| x.clone())
                            .collect::<HashSet<String>>()
                    })
                    .fold1(|p, c| {
                        p.intersection(&c)
                            .map(|x| x.clone())
                            .collect::<HashSet<String>>()
                    })
                    .unwrap();
                mapping.keys().for_each(|x| {
                    all.remove(x);
                });
                if all.len() != 1 {
                    eval_order.push_back(next);
                } else {
                    mapping.insert(all.into_iter().next().unwrap(), next);
                }
            }
            None => {
                break;
            }
        }
    }
    mapping
}
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        21
    }

    fn part1(&self, mut input: Vec<Input>) -> Output {
        input.sort_by_key(|x| x.ingredients.len());
        let mapping = get_dangerous_ingredients(&input);
        input
            .iter()
            .map(|x| x.ingredients.iter())
            .flatten()
            .filter(|&x| !mapping.contains_key(x))
            .count()
            .into()
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        input.sort_by_key(|x| x.ingredients.len());
        let mapping = get_dangerous_ingredients(&input);
        mapping
            .iter()
            .sorted_by_key(|x| x.1)
            .map(|x| x.0)
            .join(",")
            .into()
    }

    fn part1_answer(&self) -> Output {
        2734.into()
    }

    fn part2_answer(&self) -> Output {
        "kbmlt,mrccxm,lpzgzmk,ppj,stj,jvgnc,gxnr,plrlg"
            .to_string()
            .into()
    }
}
