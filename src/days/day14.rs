use crate::ParsingError;
use itertools::Itertools;
use std::str::FromStr;
use std::{collections::HashMap, convert::Infallible};

use crate::DayTrait;
type Input = Op;
type Output = u64;

#[derive(Debug, Clone)]
pub enum Op {
    SetMask(String),
    WriteMem(u64, u64),
}

impl FromStr for Op {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" = ").collect_vec();
        Ok(match parts[0] {
            "mask" => Op::SetMask(parts[1].parse()?),
            _ => Op::WriteMem(parts[0][4..parts[0].len() - 1].parse()?, parts[1].parse()?),
        })
    }
}

#[derive(Debug)]
struct MaskIter {
    set_mask: u64,
    floating: Vec<u64>,
    current: u64,
}
impl MaskIter {
    fn new(mask: &str) -> Self {
        let set_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
        let floating = mask
            .chars()
            .enumerate()
            .filter(|x| x.1 == 'X')
            .map(|x| 35u64 - x.0 as u64)
            .collect_vec();
        Self {
            set_mask,
            floating,
            current: 0,
        }
    }
    fn reset(&mut self) {
        self.current = 0;
    }
}

impl Iterator for &mut MaskIter {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current as usize) < (1 << self.floating.len()) {
            let mut and_mask = 0xffffffffffffffffu64;
            let mut or_mask = 0u64;
            let mut tmp = self.current;
            self.current += 1;
            for bit_off in &self.floating {
                let bit = tmp & 1;
                tmp >>= 1;
                match bit {
                    0 => and_mask &= !(1 << bit_off),
                    1 => or_mask |= 1 << bit_off,
                    _ => unreachable!(),
                }
            }
            Some((and_mask, or_mask | self.set_mask))
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        14
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut mask_and = 0xffffffff;
        let mut mask_or = 0;
        for line in input {
            match line {
                Op::SetMask(m) => {
                    mask_and =
                        u64::from_str_radix(&m.replace("X", "1"), 2).unwrap() | 0xfffffff << 36;
                    mask_or = u64::from_str_radix(&m.replace("X", "0"), 2).unwrap();
                }
                Op::WriteMem(dst, val) => {
                    mem.insert(dst, val & mask_and | mask_or);
                }
            }
        }
        //dbg!(mask_or, mask_and, &mem);
        mem.values().sum()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        //let mut mask_and = 0xffffffff;
        //let mut mask_or = 0;
        let mut mask = MaskIter::new("0");

        for line in &input {
            match line {
                Op::SetMask(m) => {
                    mask = MaskIter::new(&m);
                }
                Op::WriteMem(dst, val) => {
                    mask.reset();
                    mask.for_each(|(mask_and, mask_or)| {
                        mem.insert(dst & mask_and | mask_or, *val);
                    });
                }
            }
        }
        //dbg!(mask_or, mask_and, &mem);
        mem.values().sum()
    }

    fn part1_answer(&self) -> Output {
        13556564111697
    }

    fn part2_answer(&self) -> Output {
        4173715962894
    }
}
