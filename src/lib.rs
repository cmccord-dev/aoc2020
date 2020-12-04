use std::char::ParseCharError;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub enum ParsingError {
    IntError(ParseIntError),
    CharError(ParseCharError),
    ErrorWithMessage(String),
}
impl From<ParseIntError> for ParsingError {
    fn from(a: ParseIntError) -> Self {
        Self::IntError(a)
    }
}
impl From<ParseCharError> for ParsingError {
    fn from(a: ParseCharError) -> Self {
        Self::CharError(a)
    }
}
impl From<Infallible> for ParsingError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
impl From<String> for ParsingError {
    fn from(a: String) -> Self {
        Self::ErrorWithMessage(a)
    }
}
pub fn parse_list<T: FromStr>(filename: &str) -> Result<Vec<T>, T::Err> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.lines()
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
}

pub fn parse_list_delim<T: FromStr>(filename: &str, delim: &str) -> Result<Vec<T>, T::Err> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.split(delim).map(|x| x.parse::<T>()).try_collect()
}
