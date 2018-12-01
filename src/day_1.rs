use std::{collections::HashSet, fs};

pub fn day_1_part_1() -> isize {
    let freqs_text = fs::read_to_string("./day_inputs/day_1_part_1.txt").unwrap();

    freqs_text
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .sum()
}

pub fn day_1_part_2() -> Option<isize> {
    let freqs_text = fs::read_to_string("./day_inputs/day_1_part_1.txt").unwrap();
    let mut freqs_seen = HashSet::new();

    let mut sum = 0;

    for line in freqs_text
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .cycle()
    {
        sum += line;

        if !freqs_seen.insert(sum) {
            return Some(sum);
        }
    }

    None
}
