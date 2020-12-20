use crate::ParsingError;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::DayTrait;
type Input = Tile;
type Output = usize;

#[derive(Debug, Clone)]
pub struct Tile {
    id: usize,
    image: Vec<Vec<char>>,
    side_a: Vec<u32>, //top, right, bottom, left
    side_b: Vec<u32>, //top, right, bottom, left but when inverted?
}

impl Tile {
    fn rotate(&mut self) {
        self.side_a.rotate_right(1);
        self.side_b.rotate_right(1);
        //let mut next = self.image.clone();
        let first = self.image.iter().map(|x| x.iter().join("")).collect_vec();
        let N = 10;
        for i in 0..N / 2 {
            for j in i..N - 1 - i {
                let temp = self.image[i][j];
                self.image[i][j] = self.image[N - 1 - j][i];
                self.image[N - 1 - j][i] = self.image[N - 1 - i][N - 1 - j];
                self.image[N - 1 - i][N - 1 - j] = self.image[j][N - 1 - i];
                self.image[j][N - 1 - i] = temp;
            }
        }
        let second = self.image.iter().map(|x| x.iter().join("")).collect_vec();
        println!(
            "Rotate: \n{}\n",
            first
                .iter()
                .zip(second.iter())
                .map(|x| format!("{} {}", x.0, x.1))
                .join("\n")
        );
        /*for y in 0..10 {
            for x in 0..10 {
                next[x][9-y] = self.image[y][x];
            }
        }*/
        //self.image = next;
    }
    fn flip_h(&mut self) {
        //self.side_a.swap(1, 3);
        //self.side_b.swap(1, 3);
        std::mem::swap(&mut self.side_a[0], &mut self.side_b[0]);
        std::mem::swap(&mut self.side_a[2], &mut self.side_b[2]);
        self.side_a.swap(1,3);
        self.side_b.swap(1,3);
        let first = self.image.iter().map(|x| x.iter().join("")).collect_vec();
        self.image.iter_mut().for_each(|x| x.reverse());
        
        let second = self.image.iter().map(|x| x.iter().join("")).collect_vec();
        println!(
            "Horizontal: \n{}\n",
            first
                .iter()
                .zip(second.iter())
                .map(|x| format!("{} {}", x.0, x.1))
                .join("\n")
        );
    }
    fn flip_v(&mut self) {
        std::mem::swap(&mut self.side_a[1], &mut self.side_b[1]);
        std::mem::swap(&mut self.side_a[3], &mut self.side_b[3]);
        self.side_a.swap(0,2);
        self.side_b.swap(0,2);
        let first = self.image.iter().map(|x| x.iter().join("")).collect_vec();

        self.image.reverse();
        let second = self.image.iter().map(|x| x.iter().join("")).collect_vec();
        println!(
            "Vertical: \n{}\n",
            first
                .iter()
                .zip(second.iter())
                .map(|x| format!("{} {}", x.0, x.1))
                .join("\n")
        );
    }
}

impl FromStr for Tile {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .trim_end_matches(':')
            .parse()?;
        let image = lines.map(|x| x.chars().collect_vec()).collect_vec();
        let mut side_a = vec![0u32; 4];
        for i in 0..10 {
            side_a[0] = (side_a[0] << 1) | if image[0][i] == '#' { 1 } else { 0 };
            side_a[1] = (side_a[1] << 1) | if image[i][9] == '#' { 1 } else { 0 };
            side_a[2] = (side_a[2] << 1) | if image[9][i] == '#' { 1 } else { 0 };
            side_a[3] = (side_a[3] << 1) | if image[i][0] == '#' { 1 } else { 0 };
        }

        let side_b = side_a
            .iter()
            .map(|x| x.reverse_bits() >> (32 - 10))
            .collect_vec();

        Ok(Self {
            id,
            image,
            side_a,
            side_b,
        })
    }
}

fn copy_image(
    image: &mut Vec<Vec<char>>,
    off: (usize, usize),
    input: &Vec<Vec<char>>,
    rotate: bool,
    flip_v: bool,
    flip_h: bool,
) {
    let amount = 10;
    let input_x = 0;
    let input_y = 0;
    let base_x = off.0 * amount;
    let base_y = off.1 * amount;
    for x in 0..amount {
        for y in 0..amount {
            let mut thisx = x;
            let mut thisy = y;
            if rotate {
                std::mem::swap(&mut thisx, &mut thisy);
            }
            if flip_v {
                thisy = amount - 1 - thisy;
            }
            if flip_h {
                thisx = amount - 1 - thisx;
            }
            image[base_y + thisy][base_x + thisx] = input[thisy + input_x][thisx + input_y];
        }
    }
}

#[derive(Default)]
pub struct Day {}
impl DayTrait<Input, Output> for Day {
    fn get_num(&self) -> usize {
        20
    }

    fn part1(&self, input: Vec<Input>) -> Output {
        let all = input
            .iter()
            .map(|x| {
                x.side_a
                    .iter()
                    .chain(x.side_b.iter())
                    .map(|x| *x)
                    .collect_vec()
            })
            .flatten()
            .collect_vec();
        let mut map: HashMap<u32, usize> = HashMap::new();
        all.iter().for_each(|x| *map.entry(*x).or_insert(0) += 1);
        let uniq: HashSet<u32> = all.iter().unique().map(|x| *x).collect();
        let tiles = input
            .iter()
            .filter(|x| (x.side_a.iter().filter(|s| map[s] == 1).count()) == 2)
            .map(|x| x.id)
            .collect_vec();
        dbg!(&tiles);
        tiles.iter().product()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let mut tiles: HashMap<usize, Tile> = input.into_iter().map(|x| (x.id, x)).collect();
        let all = tiles
            .values()
            .map(|x| {
                x.side_a
                    .iter()
                    .chain(x.side_b.iter())
                    .map(|x| *x)
                    .collect_vec()
            })
            .flatten()
            .collect_vec();
        let mut side_map: HashMap<u32, usize> = HashMap::new();
        all.iter()
            .for_each(|x| *side_map.entry(*x).or_insert(0) += 1);
        let corners = tiles
            .values()
            .filter(|x| (x.side_a.iter().filter(|s| side_map[s] == 1).count()) == 2)
            .map(|x| x.id)
            .collect_vec();
        let edges = tiles
            .values()
            .filter(|x| (x.side_a.iter().filter(|s| side_map[s] == 1).count()) == 1)
            .map(|x| x.id)
            .collect_vec();
        //dbg!(&edges.iter().map(|x| x.id).collect_vec());
        //dbg!(&corners.iter().map(|x| x.id).collect_vec());

        let mut image: HashMap<(usize, usize), Tile> = HashMap::new();
        //let first = corners.pop().unwrap();
        //input.remove(input.find_index)
        //let mut first = tiles.remove(&corners[0]).unwrap();
        let mut first = tiles.remove(&2971).unwrap();
        while side_map[&first.side_a[0]] != 1 && side_map[&first.side_a[3]] != 1 {
            first.rotate()
        }
        image.insert((0, 0), first);
        let mut loc = (0usize, 0usize);
        let mut next_loc = (1usize, 0usize);
        let ind = 3;
        let edge_ind = 0;
        let edge_count = 2;
        let corner_count = 2;
        for c in 0..edge_count {
            let tile = &image[&loc];
            let value = &tile.side_a[(ind + 2) % 4];
            dbg!(&tile.id, &value, &c);
            dbg!(&tile.side_a);
            let next_id = tiles
                .values()
                .find(|x| x.side_a.iter().chain(x.side_b.iter()).any(|y| y == value))
                .unwrap()
                .id;
            let mut next = tiles.remove(&next_id).unwrap();
            dbg!(&next);
            if next.side_b.iter().any(|x| x == value) {
                next.flip_h();
                next.flip_v();
            }
            while &next.side_a[ind] != value {
                next.rotate()
            }
            if side_map[&next.side_a[edge_ind]] != 1 {
                if edge_ind % 2 == 0 {
                    next.flip_v()
                } else {
                    next.flip_h()
                }
            }
            image.insert(next_loc.clone(), next);
            loc = next_loc.clone();
            next_loc.0 += 1;
            for y in 0..10 {
                for i in 0..next_loc.0 {
                    print!("{} ", image[&(i, 0)].image[y].iter().join(""));
                }
                println!();
            }
        }

        //let mut image: Vec<Vec<char>> = vec![vec!['.'; 12 * 10]; 12 * 10];
        //let first = corners[0];
        //copy_image(&mut image, (0, 0), &first.image, false, false, false);

        0
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
