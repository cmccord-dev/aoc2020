use std::convert::Infallible;
use std::char::ParseCharError;
use std::num::ParseIntError;
use std::str::FromStr;


#[derive(Debug)]
pub enum ParsingError {
    IntError(ParseIntError),
    CharError(ParseCharError),
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
pub fn parse_list<T: FromStr>(filename: &str) -> Result<Vec<T>, T::Err> {
    let file = std::fs::read_to_string(filename).unwrap();
    file.split("\n")
        .filter(|line| line.len() > 0)
        .map(|x| x.parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
}
