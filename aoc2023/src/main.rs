#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;

use a1::*;
use a2::*;
use a3::*;
use a4::*;

mod a1;
mod a2;
mod a3;
mod a4;

fn main() {
    a4_2()
}

fn read(day: usize) -> String {
    fs::read_to_string(format!("inputs/{day}")).unwrap()
}

fn lines(day: usize) -> Vec<String> {
    read(day).split('\n').map(String::from).collect()
}
