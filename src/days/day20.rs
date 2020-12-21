use crate::ParsingError;
use itertools::Itertools;
use std::collections::HashMap;
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

fn rotate(a: &mut Vec<Vec<char>>) {
    let n = a.len();
    for i in 0..n / 2 {
        for j in i..n - 1 - i {
            let temp = a[i][j];
            a[i][j] = a[n - 1 - j][i];
            a[n - 1 - j][i] = a[n - 1 - i][n - 1 - j];
            a[n - 1 - i][n - 1 - j] = a[j][n - 1 - i];
            a[j][n - 1 - i] = temp;
        }
    }
}

impl Tile {
    fn rotate(&mut self) {
        self.side_a.rotate_right(1);
        self.side_b.rotate_right(1);
        rotate(&mut self.image);
    }
    fn flip_h(&mut self) {
        std::mem::swap(&mut self.side_a[0], &mut self.side_b[0]);
        std::mem::swap(&mut self.side_a[2], &mut self.side_b[2]);
        std::mem::swap(&mut self.side_a[1], &mut self.side_b[3]);
        std::mem::swap(&mut self.side_a[3], &mut self.side_b[1]);
        self.image.iter_mut().for_each(|x| x.reverse());
    }
    fn flip_v(&mut self) {
        std::mem::swap(&mut self.side_a[1], &mut self.side_b[1]);
        std::mem::swap(&mut self.side_a[3], &mut self.side_b[3]);
        std::mem::swap(&mut self.side_a[0], &mut self.side_b[2]);
        std::mem::swap(&mut self.side_a[2], &mut self.side_b[0]);
        self.image.reverse();
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
            side_a[0] = (side_a[0] << 1) | if image[0][i] == '#' { 1 } else { 0 }; //top
            side_a[1] = (side_a[1] << 1) | if image[i][9] == '#' { 1 } else { 0 }; //right
            side_a[2] = (side_a[2] << 1) | if image[9][9 - i] == '#' { 1 } else { 0 }; //bottom
            side_a[3] = (side_a[3] << 1) | if image[9 - i][0] == '#' { 1 } else { 0 };
            //left
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
    spacing: usize,
) {
    let amount = input.len() - spacing * 2;
    let base_x = off.0 * amount;
    let base_y = off.1 * amount;
    for x in 0..amount {
        for y in 0..amount {
            let thisx = x;
            let thisy = y;
            image[base_y + thisy][base_x + thisx] = input[thisy + spacing][thisx + spacing];
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
        let tiles = input
            .iter()
            .filter(|x| (x.side_a.iter().filter(|s| map[s] == 1).count()) == 2)
            .map(|x| x.id)
            .collect_vec();
        tiles.iter().product()
    }

    fn part2(&self, input: Vec<Input>) -> Output {
        let dim = (input.len() as f64).sqrt() as i32;
        let mut tiles: HashMap<usize, Tile> = input.into_iter().map(|x| (x.id, x)).collect();
        let all = tiles
            .values()
            .map(|x| {
                x.side_a
                    .iter()
                    .chain(x.side_b.iter())
                    .map(|v| (*v, x.id))
                    .collect_vec()
            })
            .flatten()
            .collect_vec();
        let mut side_map: HashMap<u32, usize> = HashMap::new();
        let mut side_map2: HashMap<u32, Vec<usize>> = HashMap::new();
        all.iter().for_each(|x| {
            *side_map.entry(x.0).or_insert(0) += 1;
            side_map2.entry(x.0).or_insert_with(|| Vec::new()).push(x.1);
        });
        let corners = tiles
            .values()
            .filter(|x| (x.side_a.iter().filter(|s| side_map[s] == 1).count()) == 2)
            .map(|x| x.id)
            .collect_vec();
        let non_unique = side_map.iter().filter(|x| x.1 > &2).collect_vec();
        assert_eq!(non_unique.len(), 0);

        let mut image: HashMap<(i32, i32), Tile> = HashMap::new();
        let mut first = tiles.remove(&corners[0]).unwrap();
        while side_map[&first.side_a[0]] != 1 || side_map[&first.side_a[3]] != 1 {
            first.rotate()
        }
        assert_eq!(side_map[&first.side_a[0]], 1);
        assert_eq!(side_map[&first.side_a[3]], 1);
        image.insert((0, 0), first);
        let left_ind = 3;
        let up_ind = 0;
        for y in 0..dim {
            for x in 0..dim {
                if y == 0 && x == 0 {
                    continue;
                }
                let above = (x, y - 1);
                let loc = (x - 1, y);
                let next_loc = (x, y);
                let up = image.get(&above).map(|x| x.side_a[(up_ind + 2) % 4]);
                let left = image.get(&loc).map(|x| x.side_a[(left_ind + 2) % 4]);
                let next = match (&left, &up) {
                    (Some(left), Some(up)) => {
                        let next_id = side_map2[left]
                            .iter()
                            .filter(|&x| image.get(&loc).unwrap().id != *x)
                            .next()
                            .unwrap();
                        let mut next = tiles.remove(&next_id).unwrap();
                        while &next.side_a[left_ind] != left && &next.side_b[left_ind] != left {
                            next.rotate()
                        }
                        if &next.side_a[left_ind] != left {
                            next.flip_v();
                        }
                        if &next.side_a[up_ind] != up {
                            next.flip_h();
                            next.rotate();
                            next.rotate();
                        }
                        next
                    }
                    (Some(left), None) => {
                        let next_id = side_map2[left]
                            .iter()
                            .filter(|&x| image.get(&loc).unwrap().id != *x)
                            .next()
                            .unwrap();
                        let mut next = tiles.remove(&next_id).unwrap();
                        if next.side_b.iter().any(|x| x == left) {
                            next.flip_h();
                            next.flip_v();
                        }
                        while &next.side_a[left_ind] != left && &next.side_b[left_ind] != left {
                            next.rotate()
                        }
                        if &next.side_b[left_ind] == left {
                            next.flip_v();
                        }
                        if side_map[&next.side_a[up_ind]] != 1 {
                            next.flip_h();
                            next.rotate();
                            next.rotate();
                        }
                        next
                    }
                    (None, Some(up)) => {
                        let next_id = side_map2[up].iter().filter(|&x| image.get(&above).unwrap().id!=*x).next().unwrap();
                        let mut next = tiles.remove(&next_id).unwrap();
                        if next.side_b.iter().any(|x| x == up) {
                            next.flip_h();
                            next.flip_v();
                        }
                        while &next.side_a[up_ind] != up && &next.side_b[up_ind] != up {
                            next.rotate()
                        }
                        if &next.side_b[up_ind] == up {
                            next.flip_h();
                        }
                        if side_map[&next.side_a[left_ind]] != 1 {
                            next.flip_v();
                            next.rotate();
                            next.rotate();
                        }

                        next
                    }
                    (None, None) => {
                        panic!();
                    }
                };
                image.insert(next_loc.clone(), next);
            }
        }
        let spacing = 1;
        let mut together =
            vec![vec!['#'; dim as usize * (10 - 2 * spacing)]; dim as usize * (10 - 2 * spacing)];

        for y in 0..dim {
            for x in 0..dim {
                copy_image(
                    &mut together,
                    (x as usize, y as usize),
                    &image[&(x, y)].image,
                    spacing,
                )
            }
        }
        let monster = r"
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "
            .lines()
            .skip(1)
            .enumerate()
            .map(|y| {
                y.1.chars()
                    .enumerate()
                    .filter(|x| x.1 == '#')
                    .map(move |x| (x.0, y.0))
            })
            .flatten()
            .collect_vec();

        let len = together.len();
        let mut count: usize = 0;
        let mut found = false;
        for i in 0..16 {
            for x in 0..len - 20 {
                for y in 0..len - 3 {
                    if monster
                        .iter()
                        .all(|(mx, my)| together[y + my][x + mx] == '#')
                    {
                        found = true;
                        count += 1;
                    }
                }
            }
            if found {
                return together
                    .iter()
                    .map(|x| x.iter().filter(|y| **y == '#').count())
                    .sum::<usize>()
                    - count * monster.len();
            }
            if i == 4 || i == 12 {
                together.reverse();
            }
            if i == 8 {
                together.iter_mut().for_each(|x| x.reverse());
            }
            rotate(&mut together);
        }

        panic!("didn't find answer");
    }

    fn part1_answer(&self) -> Output {
        83775126454273
    }

    fn part2_answer(&self) -> Output {
        1993
    }

    fn read_input(&self) -> Result<Vec<Input>, ParsingError> {
        crate::parse_list_delim(&format!("input/day{}.in", self.get_num()), "\n\n")
    }
}
