#[allow(unused_macros)]
macro_rules! dbg {
    ($($var:ident),*) => {
        #[cfg(debug_assertions)]
        println!(concat!("File: ", file!(), ", Line: ", line!(), " => ", $(stringify!($var), " = {:?}, "),*), $($var),*);
    };
}

mod day_1;
mod day_2;
mod day_3;
mod day_4;

use aoc_runner_derive::aoc_lib;

aoc_lib! { year = 2018 }
