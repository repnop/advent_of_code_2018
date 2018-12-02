use std::collections::HashMap;

pub fn day_2_part_1() -> u32 {
    let input = include_str!("../day_inputs/day_2_part_1.txt");

    let mut contains_2 = 0;
    let mut contains_3 = 0;
    let mut letters = HashMap::new();

    for line in input.lines() {
        line.chars()
            .for_each(|c| *letters.entry(c).or_insert(0) += 1);
        contains_2 += if letters.values().filter(|v| **v == 2).count() > 0 {
            1
        } else {
            0
        };
        contains_3 += if letters.values().filter(|v| **v == 3).count() > 0 {
            1
        } else {
            0
        };

        letters.clear();
    }

    contains_2 * contains_3
}

pub fn day_2_part_2() -> String {
    let input = include_str!("../day_inputs/day_2_part_1.txt");

    for (i, line) in input.lines().enumerate() {
        for line2 in input.lines().skip(i) {
            if hamming(line, line2) == 1 {
                return line
                    .chars()
                    .zip(line2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c1, _)| c1)
                    .collect();
            }
        }
    }

    String::new()
}

fn hamming(s1: &str, s2: &str) -> u32 {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| if c1 != c2 { 1 } else { 0 })
        .sum()
}
