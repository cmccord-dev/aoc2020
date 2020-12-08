use crate::ParsingError;
use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

use crate::DayTrait;
type Output = i64;

type Input = Instr;

#[derive(Debug, Clone)]
pub struct Instr {
    instr: String,
    val: i64,
}

impl FromStr for Instr {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect_vec();
        Ok(Instr {
            instr: parts[0].parse()?,
            val: parts[1].parse()?,
        })
    }
}

fn run_to_end(input: &Vec<Input>) -> Option<i64> {
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    let mut count = 0;
    loop {
        let instr = &input[pc as usize];
        pc += 1;
        
        match instr.instr.as_str() {
            "nop" => (),
            "acc" => acc = acc + instr.val,
            "jmp" => pc = pc + instr.val - 1,
            _ => unimplemented!()
        };
        count += 1;
        if pc < 0 {
            return None
        }
        if count >= input.len() {
            return None
        }
        if pc as usize == input.len() {
            return Some(acc)
        }
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        8
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut data = input.into_iter().map(|x| (x, false)).collect_vec();
        let mut pc: i64 = 0;
        let mut acc = 0;
        loop {
            let instr = &mut data[pc as usize];
            pc += 1;
            if instr.1 {
                break;
            }
            instr.1 = true;
            
            match instr.0.instr.as_str() {
                "nop" => (),
                "acc" => acc = acc + instr.0.val,
                "jmp" => pc = pc + instr.0.val - 1,
                _ => unimplemented!()
            };

        }
        acc
    }

    fn part2(&self, mut input: Vec<Input>) -> Output {
        let mut nop = String::from("nop");
        let mut jmp = String::from("jmp");
        for i in 0..input.len() {
            match input[i].instr.as_str() {
                "nop" => {
                    std::mem::swap(&mut input[i].instr, &mut jmp);
                    if let Some(val)  = run_to_end(&input){
                        return val
                    }
                    std::mem::swap(&mut input[i].instr, &mut jmp);
                }
                "jmp" => {
                    std::mem::swap(&mut input[i].instr, &mut nop);
                    if let Some(val)  = run_to_end(&input){
                        return val
                    }
                    std::mem::swap(&mut input[i].instr, &mut nop);
                }
                _ => (),
            }
        }
        0
    }

    fn part1_answer(&self) -> Output {
        1753
    }

    fn part2_answer(&self) -> Output {
        733
    }
}
