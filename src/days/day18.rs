use crate::ParsingError;
use itertools::Itertools;
use std::str::FromStr;

use crate::DayTrait;
//type Input = u64;
type Output = i64;

#[derive(Debug, Clone)]
pub enum Token {
    Value(i64),
    Operator(char),
}

#[derive(Debug, Clone)]
pub struct Input(Vec<Token>);

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            s.chars()
                .filter(|x| !x.is_whitespace())
                .map(|x| match x {
                    '0'..='9' => Token::Value(x as i64 - '0' as i64),
                    _ => Token::Operator(x),
                })
                .collect_vec(),
        ))
    }
}

fn prec(c: char) -> (char, i32) {
    (
        c,
        match c {
            '(' => 0,
            '+' => 2,
            '*' => 1,
            _ => 3,
        },
    )
}
fn eval2(input: &Vec<Token>) -> i64 {
    let mut vals = Vec::new();
    let mut ops = Vec::new();
    //dbg!(&input);
    input.iter().for_each(|c| {
        //dbg!(&vals, &ops, &c);
        match c {
            Token::Value(val) => {
                vals.push(*val);
            }
            Token::Operator('(') => {
                ops.push(('(', 0));
            }
            Token::Operator(')') => loop {
                match ops.pop().unwrap().0 {
                    '*' => {
                        let a = vals.pop().unwrap();
                        let b = vals.pop().unwrap();
                        vals.push(a * b);
                    }
                    '+' => {
                        let a = vals.pop().unwrap();
                        let b = vals.pop().unwrap();
                        vals.push(a + b);
                    }
                    '(' => break,
                    _ => panic!(),
                }
            },
            Token::Operator(op) => match ops.last() {
                None => ops.push(prec(*op)),
                Some(('(', _)) => ops.push(prec(*op)),
                Some(_) => {
                    let p = prec(*op).1;
                    //dbg!(&p, &ops, &vals);
                    while ops.len() > 0 && ops.last().unwrap().1 >= p {
                        let a = vals.pop().unwrap();
                        let b = vals.pop().unwrap();
                        let last_op = ops.pop().unwrap().0;
                        vals.push(match last_op {
                            '*' => a * b,
                            '+' => a + b,
                            _ => unimplemented!(),
                        });
                    }
                    ops.push(prec(*op))
                }
            },
        }
    });
    while ops.len() > 0 {
        match ops.pop().unwrap().0 {
            '*' => {
                let a = vals.pop().unwrap();
                let b = vals.pop().unwrap();
                vals.push(a * b);
            }
            '+' => {
                let a = vals.pop().unwrap();
                let b = vals.pop().unwrap();
                vals.push(a + b);
            }
            _ => panic!(),
        }
    }
    vals.pop().unwrap()
}

fn eval(input: &Vec<Token>) -> i64 {
    let mut vals = Vec::new();
    let mut ops = Vec::new();
    //dbg!(&input);
    input.iter().for_each(|c| {
        //dbg!(&vals, &ops, &c);
        match c {
            Token::Value(val) => {
                vals.push(*val);
            }
            Token::Operator('(') => {
                ops.push('(');
            }
            Token::Operator(')') => loop {
                match ops.pop().unwrap() {
                    '*' => {
                        let a = vals.pop().unwrap();
                        let b = vals.pop().unwrap();
                        vals.push(a * b);
                    }
                    '+' => {
                        let a = vals.pop().unwrap();
                        let b = vals.pop().unwrap();
                        vals.push(a + b);
                    }
                    '(' => break,
                    _ => panic!(),
                }
            },
            Token::Operator(op) => match ops.last() {
                None => ops.push(*op),
                Some('(') => ops.push(*op),
                Some(_) => {
                    let a = vals.pop().unwrap();
                    let b = vals.pop().unwrap();
                    let last_op = ops.pop().unwrap();
                    vals.push(match last_op {
                        '*' => a * b,
                        '+' => a + b,
                        _ => unimplemented!(),
                    });
                    ops.push(*op)
                }
            },
        }
    });
    while ops.len() > 0 {
        match ops.pop().unwrap() {
            '*' => {
                let a = vals.pop().unwrap();
                let b = vals.pop().unwrap();
                vals.push(a * b);
            }
            '+' => {
                let a = vals.pop().unwrap();
                let b = vals.pop().unwrap();
                vals.push(a + b);
            }
            _ => panic!(),
        }
    }
    vals.pop().unwrap()
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        18
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        input.iter().map(|x| eval(&x.0)).sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        input.iter().map(|x| eval2(&x.0)).sum()
    }

    fn part1_answer(&self) -> Output {
        3885386961962
    }

    fn part2_answer(&self) -> Output {
        0
    }
}
