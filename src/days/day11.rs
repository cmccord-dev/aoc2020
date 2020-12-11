use crate::{input_struct, time, ParsingError};
use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Instant;

use crate::DayTrait;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Filled,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Filled,
            '.' | _ => Seat::Floor,
        }
    }
}

impl Seat {
    fn next(&self, num_adjancent: usize, max_adjacent: usize) -> Self {
        match self {
            Self::Floor => Self::Floor,
            Self::Filled => {
                if num_adjancent < max_adjacent {
                    Self::Filled
                } else {
                    Self::Empty
                }
            }
            Self::Empty => {
                if num_adjancent == 0 {
                    Self::Filled
                } else {
                    Self::Empty
                }
            }
        }
    }
}

input_struct!(Input, Vec<Seat>);

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Input(s.chars().map(|x| x.into()).collect_vec()))
    }
}

type Output = usize;

fn print_arr(input: &Vec<Vec<Seat>>) {
    println!(
        "{}",
        input
            .iter()
            .map(|x| x
                .iter()
                .map(|y| match y {
                    Seat::Filled => '#',
                    Seat::Empty => 'L',
                    Seat::Floor => '.',
                })
                .join(""))
            .join("\n")
    )
}

//input_struct!(Seats, Vec<Vec<Seat>>);

fn find_stable(input: Vec<Vec<Seat>>, adj: Vec<Vec<Vec<(usize, usize)>>>, max_adj: usize) -> usize {
    let w = input[0].len();
    let h = input.len();
    let mut a = input;
    let mut b = a.clone();
    loop {
        let mut ident_count = 0;
        for i in 0..h {
            for j in 0..w {
                match a[i][j] {
                    Seat::Floor => (),
                    _ => {
                        /*let count = adj[i][j].iter().fold(0, |p, c| {
                            //dbg!(&i,&j,&c);
                            if let Seat::Filled = a[c.0][c.1] {
                                p + 1
                            } else {
                                p
                            }
                        });*/
                        let mut count = 0;
                        for c in adj[i][j].iter() {
                            if let Seat::Filled = a[c.0][c.1] {
                                count += 1;
                            }
                        }
                        b[i][j] = match a[i][j] {
                            Seat::Filled => {
                                if count < max_adj {
                                    Seat::Filled
                                } else {
                                    Seat::Empty
                                }
                            }
                            Seat::Empty => {
                                if count == 0 {
                                    Seat::Filled
                                } else {
                                    Seat::Empty
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                if a[i][j] == b[i][j] {
                    ident_count += 1;
                }
            }
        }
        if ident_count == w * h {
            break;
        }
        std::mem::swap(&mut a, &mut b);
        //dbg!(&ident_count);
        //print_arr(&a);
    }
    a.iter()
        .map(|x| -> usize {
            x.iter()
                .map(|c| if let Seat::Filled = c { 1usize } else { 0usize })
                .sum()
        })
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Seats(Vec<Vec<Seat>>);

impl Seats {
    fn get_adjacent1(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(move |x| (pos.0 as i32 + x.0, pos.1 as i32 + x.1))
        .map(move |x| {
            if x.0 >= 0
                && x.1 >= 0
                && (x.0 as usize) < self.0.len()
                && (x.1 as usize) < self.0[0].len()
            {
                Some((x.0 as usize, x.1 as usize))
            } else {
                None
            }
        })
        .flatten()
        .collect_vec()
    }
    fn get_adjacent2<'a>(&'a self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(move |x| {
            let mut sx = pos.0 as i32 + x.0;
            let mut sy = pos.1 as i32 + x.1;
            loop {
                if sx >= 0
                    && sy >= 0
                    && (sx as usize) < self.0.len()
                    && (sy as usize) < self.0[0].len()
                {
                    match self.0[sx as usize][sy as usize] {
                        Seat::Floor => {
                            sx += x.0;
                            sy += x.1;
                        }
                        _ => break Some((sx as usize, sy as usize)),
                    }
                } else {
                    break None;
                }
            }
        })
        .flatten()
        .collect_vec()
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        11
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let seats = Seats(input.into_iter().map(|x| x.0).collect_vec());
        let w = seats.0[0].len();
        let h = seats.0.len();
        let adj = (0..h)
            .map(|i| (0..w).map(|j| seats.get_adjacent1((i, j))).collect_vec())
            .collect_vec();
        find_stable(seats.0, adj, 4)

        /*loop {
            let next = (0..seats.0.len())
                .map(|x| {
                    (0..seats.0[x].len())
                        .map(|y| {
                            seats.0[x][y].next(
                                seats
                                    .get_adjacent1((x, y))
                                    .filter(|x| match x {
                                        Seat::Filled => true,
                                        _ => false,
                                    })
                                    .count(),
                                4,
                            )
                        })
                        .collect_vec()
                })
                .collect_vec();
            if next == seats.0 {
                break;
            }
            seats = Seats(next)
        }
        seats
            .0
            .iter()
            .map(|x| {
                x.iter()
                    .filter(|x| match x {
                        Seat::Filled => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()*/
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut seats = Seats(input.into_iter().map(|x| x.0).collect_vec());
        let w = seats.0[0].len();
        let h = seats.0.len();
        let adj = (0..h)
            .map(|i| (0..w).map(|j| seats.get_adjacent2((i, j))).collect_vec())
            .collect_vec();
        find_stable(seats.0, adj, 5)
        /*loop {
            let next = (0..seats.0.len())
                .map(|x| {
                    (0..seats.0[x].len())
                        .map(|y| {
                            seats.0[x][y].next(
                                seats
                                    .get_adjacent2((x, y)).into_iter().map(|x| seats.0[x.0][x.1])
                                    .filter(|x| match x {
                                        Seat::Filled => true,
                                        _ => false,
                                    })
                                    .count(),
                                5,
                            )
                        })
                        .collect_vec()
                })
                .collect_vec();
            if next == seats.0 {
                break;
            }
            seats = Seats(next)
        }
        seats
            .0
            .iter()
            .map(|x| {
                x.iter()
                    .filter(|x| match x {
                        Seat::Filled => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()*/
    }

    fn part1_answer(&self) -> Output {
        2368
    }

    fn part2_answer(&self) -> Output {
        2124
    }
}
