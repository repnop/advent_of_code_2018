use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> [i32; 300 * 300] {
    let serial = input.trim().parse::<i32>().unwrap();
    let mut v = [0; 300 * 300];

    v.par_iter_mut().enumerate().for_each(|(idx, val)| {
        let (x, y) = (idx as i32 % 300, idx as i32 / 300);

        *val = calculate_power_level(x, y, serial);
    });

    v
}

#[aoc(day11, part1)]
pub fn day_11_part_1(input: &[i32]) -> String {
    let ((x, y), _) = input
        .par_iter()
        .enumerate()
        .map(|(idx, _)| {
            let (x, y) = (idx as i32 % 300, idx as i32 / 300);
            if x >= 297 || y >= 297 {
                ((x, y), 0)
            } else {
                let top_row = (&input[(y * 300 + x) as usize..][..3]).iter().sum::<i32>();
                let mid_row = (&input[((y + 1) * 300 + x) as usize..][..3])
                    .iter()
                    .sum::<i32>();
                let bottom_row = (&input[((y + 2) * 300 + x) as usize..][..3])
                    .iter()
                    .sum::<i32>();

                ((x, y), top_row + mid_row + bottom_row)
            }
        })
        .max_by_key(|(_, v)| *v)
        .unwrap();

    format!("{}, {}", x, y)
}

#[aoc(day11, part1, no_rayon)]
pub fn day_11_part_1_no_rayon(input: &[i32]) -> String {
    let ((x, y), _) = input
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let (x, y) = (idx as i32 % 300, idx as i32 / 300);
            if x >= 297 || y >= 297 {
                ((x, y), 0)
            } else {
                let top_row = (&input[(y * 300 + x) as usize..][..3]).iter().sum::<i32>();
                let mid_row = (&input[((y + 1) * 300 + x) as usize..][..3])
                    .iter()
                    .sum::<i32>();
                let bottom_row = (&input[((y + 2) * 300 + x) as usize..][..3])
                    .iter()
                    .sum::<i32>();

                ((x, y), top_row + mid_row + bottom_row)
            }
        })
        .max_by_key(|(_, v)| *v)
        .unwrap();

    format!("{}, {}", x, y)
}

// Uses the summed-area table algorithm.
#[aoc(day11, part2)]
pub fn day_11_part_2(input: &[i32]) -> String {
    let preprocessed = preprocess(input);

    let ((x, y, n), _) = (0..300)
        .map(move |x| (0..300).map(move |y| (x, y)))
        .flatten()
        .map(|(x, y)| {
            (0..300 - x.max(y))
                .map(|n| {
                    (
                        // offset size by one because of starting at 0 I guess?
                        (x, y, n + 1),
                        get_sum(&preprocessed, x as usize, y as usize, n as usize),
                    )
                })
                .max_by_key(|(_, val)| *val)
                .unwrap()
        })
        .max_by_key(|(_, val)| *val)
        .unwrap();

    format!("{},{},{}", x, y, n)
}

fn get_sum(preprocessed: &[i32], x: usize, y: usize, size: usize) -> i32 {
    let max_y = y + size;
    let max_x = x + size;

    let mut result = preprocessed[max_y * 300 + max_x];

    if x > 0 {
        result -= preprocessed[max_y * 300 + (x - 1)];
    }

    if y > 0 {
        result -= preprocessed[(y - 1) * 300 + (x + size)];
    }

    if x > 0 && y > 0 {
        result += preprocessed[(y - 1) * 300 + (x - 1)];
    }

    result
}

fn preprocess(input: &[i32]) -> [i32; 300 * 300] {
    let mut ret = [0; 300 * 300];

    ret[..300].clone_from_slice(&input[..300]);

    for i in 1..300 {
        for j in 0..300 {
            ret[i * 300 + j] = input[i * 300 + j] + ret[(i - 1) * 300 + j];
        }
    }

    for i in 0..300 {
        for j in 1..300 {
            ret[i * 300 + j] += ret[i * 300 + (j - 1)];
        }
    }

    ret
}

fn calculate_power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = y * rack_id;

    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;

    power_level - 5
}

#[test]
fn power_level_test() {
    assert_eq!(calculate_power_level(3, 5, 8), 4);
    assert_eq!(calculate_power_level(122, 79, 57), -5);
    assert_eq!(calculate_power_level(217, 196, 39), 0);
    assert_eq!(calculate_power_level(101, 153, 71), 4);
}

#[test]
fn part_2_tests() {
    let input = generator("18");
    assert_eq!(day_11_part_2(&input), "90,269,16");

    let input = generator("42");
    assert_eq!(day_11_part_2(&input), "232,251,12");
}
