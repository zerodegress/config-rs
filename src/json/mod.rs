use std::collections::HashMap;

pub mod parser;

pub enum Number {
    Integer(i64),
    Float(f64)
}

pub enum Value {
    String(String),
    Number(Number),
    Object(HashMap<String, Value>),
    Array(Vec<Value>)
}