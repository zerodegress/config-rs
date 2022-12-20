use std::str::FromStr;

use nom::{bytes::complete::tag, combinator::{map_res, map}, IResult, character::{complete::digit1, streaming::not_line_ending}, number::complete::recognize_float, branch::alt, sequence::delimited};
use regex::Regex;

use super::{Number, Value};

pub fn integer(input: &str) -> IResult<&str, Number> {
    map_res(digit1, |s: &str| (Ok(Number::Integer(s.parse::<i64>()?)) as Result<Number, <i64 as FromStr>::Err>))(input)
}


pub fn float(input: &str) -> IResult<&str, Number> {
    map_res(recognize_float, |s: &str| (Ok(Number::Float(s.parse::<f64>()?)) as Result<Number, <f64 as FromStr>::Err>))(input)
}

pub fn number(input: &str) -> IResult<&str, Value> {
    map(alt((integer, float)), |num: Number| Value::Number(num))(input)
}

pub fn string(input: &str) -> IResult<&str, Value> {
    map(delimited(tag("\""), map(not_line_ending, |s: &str| s), tag("\"")), |s: &str| Value::String(s.to_owned()))(input)
}