use std::str::FromStr;

pub fn parse_list<T: FromStr>(filename: &str) -> Vec<T> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.split("\n")
        .filter(|line| line.len() > 0)
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>()
}
