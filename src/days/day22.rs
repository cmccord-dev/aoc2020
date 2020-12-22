use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use crate::{DayTrait, ParsingError};
type Input = Deck;
type Output = u64;

#[derive(Debug, Clone)]
pub struct Deck(Vec<u64>);

impl FromStr for Deck {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .skip(1)
            .map(|x| x.parse())
            .try_collect()
            .map(|x| Self(x))?)
    }
}

fn game(
    mut p1: VecDeque<u64>,
    mut p2: VecDeque<u64>,
    num: &mut u64,
) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut set = HashSet::new();
    let this_game = *num;
    let print = false;
    if print {
        println!("=== Game {} ===\n", this_game);
    }
    let mut round = 1;
    loop {
        if p1.len() == 0 || p2.len() == 0 {
            if print {
                println!(
                    "The winner of game {} is player {}!\n",
                    this_game,
                    if p1.len() == 0 { 2 } else { 1 }
                );
            }
            return (p1, p2);
        }
        if !set.insert((p1.clone(), p2.clone())) {
            if print {
                println!("The winner of game {} is player 1!\n", this_game);
            }
            return (p1, VecDeque::new());
        }
        if print {
            println!("-- Round {} Game {} ---", round, this_game);
            println!(
                "Player 1's deck: {}",
                p1.iter().map(|x| x.to_string()).join(", ")
            );
            println!(
                "Player 2's deck: {}",
                p2.iter().map(|x| x.to_string()).join(", ")
            );
        }
        let a = p1.pop_front().unwrap();
        let b = p2.pop_front().unwrap();
        if print {
            println!("Player 1 plays: {}", a);
            println!("Player 2 plays: {}", b);
        }
        if a as usize <= p1.len() && b as usize <= p2.len() {
            *num = *num + 1;
            if print {
                println!("Playing a sub-game to determine the winner...\n");
            }

            let res = game(
                p1.iter().take(a as usize).map(|x| *x).collect(),
                p2.iter().take(b as usize).map(|x| *x).collect(),
                num,
            );
            if print {
                println!("... anyway, back to game {}.", this_game);
            }
            match res.0.len().cmp(&res.1.len()) {
                std::cmp::Ordering::Greater => {
                    if print {
                        println!("Player 1 wins round {} of game {}!\n", round, this_game);
                    }
                    p1.push_back(a);
                    p1.push_back(b);
                }
                std::cmp::Ordering::Equal => {
                    panic!();
                }
                std::cmp::Ordering::Less => {
                    if print {
                        println!("Player 2 wins round {} of game {}!\n", round, this_game);
                    }
                    p2.push_back(b);
                    p2.push_back(a);
                }
            }
        } else {
            if a > b {
                if print {
                    println!("Player 1 wins round {} of game {}!\n", round, this_game);
                }
                p1.push_back(a);
                p1.push_back(b);
            } else {
                if print {
                    println!("Player 2 wins round {} of game {}!\n", round, this_game);
                }
                p2.push_back(b);
                p2.push_back(a);
            }
        }
        round += 1;
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        22
    }

    fn part1(&self, mut input: Vec<Input>) -> Output {
        let mut winner = VecDeque::from(input.pop().unwrap().0);
        let mut loser = VecDeque::from(input.pop().unwrap().0);
        loop {
            let a = winner.pop_front().unwrap();
            let b = loser.pop_front().unwrap();
            if a < b {
                loser.push_back(b);
                loser.push_back(a);
            } else {
                winner.push_back(a);
                winner.push_back(b);
            }
            if winner.len() < loser.len() {
                std::mem::swap(&mut winner, &mut loser);
            }
            if loser.len() == 0 {
                break;
            }
        }
        dbg!(&winner);
        winner
            .into_iter()
            .rev()
            .enumerate()
            .map(|x| (x.0 as u64 + 1) * x.1)
            .sum()
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        let p2 = VecDeque::from(input.pop().unwrap().0);
        let p1 = VecDeque::from(input.pop().unwrap().0);
        let mut num = 1;
        let res = game(p1, p2, &mut num);
        let mut winner = res.0;
        if res.1.len() > winner.len() {
            winner = res.1;
        }
        winner
            .into_iter()
            .rev()
            .enumerate()
            .map(|x| (x.0 as u64 + 1) * x.1)
            .sum()
    }

    fn part1_answer(&self) -> Output {
        0
    }

    fn part2_answer(&self) -> Output {
        0
    }

    fn read_input(&self) -> Result<Vec<Input>, ParsingError> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), "\n\n")
    }
}
