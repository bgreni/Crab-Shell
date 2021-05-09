use std::fmt;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

pub struct Output {
    data: Vec<Vec<String>>,
    headers: Vec<String>,
}

const JOIN_SPACE: &str = "     ";

impl Output {
    fn new() -> Output {
        return Output {
            data: Vec::new(),
            headers: Vec::new(),
        }
    }

    fn to_string(&self) -> String {
        let mut out = self.headers.join(JOIN_SPACE);

    }
}


impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}