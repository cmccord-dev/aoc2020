use std::collections::HashMap;
use crate::ParsingError;
use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

use crate::DayTrait;
type Output = usize;

type Input = Bag;

#[derive(Debug, Clone)]
pub struct Bag {
    //parents: Vec<String>,
    children: Vec<(String, usize)>,
    name: String,
    contains_gold: Option<bool>,
}

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        let name = format!("{} {}", words.next().unwrap(), words.next().unwrap());
        let mut children = Vec::new();
        words.next();
        words.next();
        loop {
            match words.next() {
                None | Some("no") => break,

                Some(t) => {
                    children.push((
                        format!("{} {}", words.next().unwrap(), words.next().unwrap()),
                        t.parse()?,
                    ));
                    words.next();
                }
            }
        }
        Ok(Bag { children, name, contains_gold: None })
    }
}

fn find_gold(current: &str, bags: &mut HashMap<String, Bag>) -> bool {
    let bag = &bags[current];
    if let Some(v) = bag.contains_gold {
        v
    }else {
        let tmp = bag.children.iter().map(|x| x.0.clone()).collect_vec();
        let res = tmp.iter().any(|x| x=="shiny gold" || find_gold(&x, bags));
        bags.get_mut(current).unwrap().contains_gold = Some(res);
        res
    }

}

fn count_children(current: &str, bags: &HashMap<String, Bag>) -> usize {
    bags[current].children.iter().map(|x| x.1*(1+count_children(&x.0, bags))).sum()
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        7
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut map: HashMap<String, Bag> = input.into_iter().map(|x| (x.name.clone(), x)).collect();
        let keys = map.keys().map(|x|x.clone()).collect_vec();
        keys.iter().filter(|x| find_gold(x, &mut map)).count()
        
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let map: HashMap<String, Bag> = input.into_iter().map(|x| (x.name.clone(), x)).collect();
        count_children("shiny gold", &map)
    }

    fn part1_answer(&self) -> Output {
        222
    }

    fn part2_answer(&self) -> Output {
        13264
    }
}
