#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;

use a1::a1;
use a2::*;

mod a1;
mod a2;

fn main() {
    a2_2()
}

fn read(day: usize) -> String {
    fs::read_to_string(format!("inputs/{day}")).unwrap()
}

fn lines(day: usize) -> Vec<String> {
    read(day).split('\n').map(String::from).collect()
}
