use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn day_1_part_1(input: &str) -> isize {
    input.lines().map(|l| l.parse::<isize>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn day_1_part_2(input: &str) -> isize {
    let mut freqs_seen = HashSet::new();

    let mut sum = 0;

    for line in input.lines().map(|l| l.parse::<isize>().unwrap()).cycle() {
        sum += line;

        if !freqs_seen.insert(sum) {
            return sum;
        }
    }

    unreachable!()
}
