use crate::ParsingError;
use itertools::Itertools;
use nalgebra::Vector3;
use std::collections::HashMap;
use std::{collections::HashSet, str::FromStr};

use crate::DayTrait;

type Vec3 = Vector3<i64>;
//type Vec4 = Vector4<i64>;

#[derive(Debug, Clone)]
pub struct Input(Vec<bool>);
type Output = usize;

impl FromStr for Input {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.chars().map(|x| x == '#').collect_vec()))
    }
}

fn get_neighbors(v: &Vec3) -> Vec<Vec3> {
    (-1..2)
        .map(|x| {
            (-1..2)
                .map(move |y| (-1..2).map(move |z| v + Vec3::new(x, y, z)))
                .flatten()
        })
        .flatten()
        .collect_vec()
}

/*fn get_neighbors2(v: &Vec4) -> Vec<Vec4> {
    (-1..2)
        .map(|x| {
            (-1..2)
                .map(move |y| {
                    (-1..2)
                        .map(move |z| (-1..2).map(move |w| v + Vec4::new(x, y, z, w)))
                        .flatten()
                })
                .flatten()
        })
        .flatten()
        .collect_vec()
}
fn print_map(map: &HashMap<Vec3, bool>) {
    let min_x = map.keys().map(|x| x[0]).min().unwrap();
    let min_y = map.keys().map(|x| x[1]).min().unwrap();
    let min_z = map.keys().map(|x| x[2]).min().unwrap();
    let max_x = map.keys().map(|x| x[0]).max().unwrap();
    let max_y = map.keys().map(|x| x[1]).max().unwrap();
    let max_z = map.keys().map(|x| x[2]).max().unwrap();

    for z in min_z..=max_z {
        println!("Z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if *map.get(&Vec3::new(x, y, z)).unwrap_or(&false) {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
            println!("");
        }
    }
}*/
#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        17
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let mut current: HashMap<Vec3, bool> = HashMap::new();
        let mut next = current.clone();
        input.iter().enumerate().for_each(|(y, v)| {
            v.0.iter().enumerate().for_each(|(x, b)| {
                current.insert(Vec3::new(x as i64, y as i64, 0), *b);
            })
        });

        for _ in 0..6 {
            //print_map(&current);
            let cells: HashSet<Vec3> = current.keys().map(|x| get_neighbors(x)).flatten().collect();

            cells.iter().for_each(|x| {
                match (
                    current.get(x).unwrap_or(&false),
                    get_neighbors(x)
                        .iter()
                        .filter(|f| x != *f && *current.get(f).unwrap_or(&false))
                        .count(),
                ) {
                    (true, 2) | (true, 3) | (false, 3) => {
                        next.insert(x.clone(), true);
                    }
                    _ => (),
                };
            });
            std::mem::swap(&mut current, &mut next);
            next.clear();
        }
        current.values().filter(|&x| *x).count()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let amount = 6 * 2 + 3;
        let ydim = input.len() + amount;
        let xdim = input[0].0.len() + amount;
        let zdim = amount;
        let wdim = amount;
        let xoff = 7;
        let yoff = 7;
        let zoff = 7;
        let woff = 7;
        let mut current2 = vec![vec![vec![vec![false; wdim]; zdim]; ydim]; xdim];

        //let mut current: HashMap<Vec4, bool> = HashMap::new();

        //let mut next = current.clone();
        let mut next2 = current2.clone();
        input.iter().enumerate().for_each(|(y, v)| {
            v.0.iter().enumerate().for_each(|(x, b)| {
                //current.insert(Vec4::new(x as i64, y as i64, 0, 0), *b);
                current2[x + xoff][y + yoff][zoff][woff] = *b;
            })
        });

        for _ in 0..6 {
            //print_map(&current);
            /*let cells: HashSet<Vec4> = current
                .keys()
                .map(|x| get_neighbors2(x))
                .flatten()
                .collect();

            cells.iter().for_each(|x| {
                match (
                    current.get(x).unwrap_or(&false),
                    get_neighbors2(x)
                        .iter()
                        .filter(|f| x != *f && *current.get(f).unwrap_or(&false))
                        .count(),
                ) {
                    (true, 2) | (true, 3) | (false, 3) => {
                        next.insert(x.clone(), true);
                    }
                    _ => (),
                };
            });*/

            for x in 1..xdim - 1 {
                for y in 1..ydim - 1 {
                    for z in 1..zdim - 1 {
                        for w in 1..wdim - 1 {
                            let mut count = 0;
                            let current = current2[x][y][z][w];
                            for xx in -1..2i64 {
                                for yy in -1..2i64 {
                                    for zz in -1..2i64 {
                                        for ww in -1..2i64 {
                                            if xx == 0 && yy == 0 && zz == 0 && ww == 0 {
                                                continue;
                                            }
                                            if current2[(x as i64 + xx) as usize]
                                                [(y as i64 + yy) as usize]
                                                [(z as i64 + zz) as usize]
                                                [(w as i64 + ww) as usize]
                                            {
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            next2[x][y][z][w] = count == 3 || (current && count == 2)
                        }
                    }
                }
            }

            //std::mem::swap(&mut current, &mut next);
            std::mem::swap(&mut current2, &mut next2);
            //next.clear();
        }
        //current.values().filter(|&x| *x).count()
        current2
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|&x| *x)
            .count()
    }

    fn part1_answer(&self) -> Output {
        273
    }

    fn part2_answer(&self) -> Output {
        1504
    }
}
