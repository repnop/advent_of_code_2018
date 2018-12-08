#[allow(unused_macros)]
macro_rules! dbg {
    ($($var:ident),*) => {
        println!(concat!("File: ", file!(), ", Line: ", line!(), " => ", $(stringify!($var), " = {:?}, "),*), $($var),*);
    };
}

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use aoc_runner_derive::aoc_lib;

aoc_lib! { year = 2018 }
