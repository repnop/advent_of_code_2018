#[allow(unused_macros)]
macro_rules! dbg {
    ($($var:ident),*) => {
        println!(concat!("File: ", file!(), ", Line: ", line!(), " => ", $(stringify!($var), " = {:?}, "),*), $($var),*);
    };
}

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

use aoc_runner_derive::aoc_lib;

aoc_lib! { year = 2018 }
