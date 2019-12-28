#![allow(dead_code)]

extern crate regex;
extern crate console;
extern crate num_format;

use std::time::SystemTime;
use num_format::{Locale, ToFormattedString};

mod intcode;
mod puzzle_01;
mod puzzle_02;
mod puzzle_03;
mod puzzle_04;
mod puzzle_05;
mod puzzle_06;
mod puzzle_07;
mod puzzle_08;
mod puzzle_09;
mod puzzle_10;
mod puzzle_11;
mod puzzle_12;
mod puzzle_13;
mod puzzle_14;
mod puzzle_15;
mod puzzle_16;
mod puzzle_18;
mod puzzle_19;

fn main() {
    let now = SystemTime::now();

    puzzle_19::run();

    let elapsed = now.elapsed().unwrap();
    let elapsted_ms = elapsed.as_millis();
    println!("Run completed in {}ms", elapsted_ms.to_formatted_string(&Locale::en));
}
