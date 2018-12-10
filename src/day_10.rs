use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(PartialEq, PartialOrd, Default, Debug, Clone, Copy)]
pub struct Point {
    id: u32,
    position: Position,
    velocity: Velocity,
}

#[derive(PartialEq, PartialOrd, Default, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, PartialOrd, Default, Debug, Clone, Copy)]
pub struct Velocity {
    dx: i32,
    dy: i32,
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Point> {
    let regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)>\s*velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let mut id = 0;
    regex
        .captures_iter(input)
        .map(|mtch| {
            id += 1;
            Point {
                id,
                position: Position {
                    x: mtch[1].parse().unwrap(),
                    y: mtch[2].parse().unwrap(),
                },
                velocity: Velocity {
                    dx: mtch[3].parse().unwrap(),
                    dy: mtch[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn day_10_part_1(input: &[Point]) -> u32 {
    use std::io::Write;

    let mut points = input.to_vec();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open("../../../day_10_output/output.txt")
        .unwrap();

    let mut s = String::new();
    for _ in 0..500_000 {
        s.clear();

        points.iter_mut().for_each(|point| {
            point.position.x += point.velocity.dx;
            point.position.y += point.velocity.dy;
        });

        let top_x = points
            .iter()
            .min_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;
        let bottom_x = points
            .iter()
            .max_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;
        let top_y = points
            .iter()
            .min_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;
        let bottom_y = points
            .iter()
            .max_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;

        if bottom_x - top_x > 70 || bottom_y - top_y > 70 {
            continue;
        }

        s.push_str("\n\n--------------------\n\n");

        for x in top_x..(bottom_x + 50) {
            for y in (top_y..=bottom_y).rev() {
                if points
                    .iter()
                    .any(|point| point.position.x == x && point.position.y == y)
                {
                    s.push('X');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }

        file.write_all(s.as_bytes()).unwrap();
    }
    0
}

#[aoc(day10, part2)]
pub fn day_10_part_2(input: &[Point]) -> u32 {
    let mut points = input.to_vec();

    for i in 1..500_000 {
        points.iter_mut().for_each(|point| {
            point.position.x += point.velocity.dx;
            point.position.y += point.velocity.dy;
        });

        let top_x = points
            .iter()
            .min_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;
        let bottom_x = points
            .iter()
            .max_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;
        let top_y = points
            .iter()
            .min_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;
        let bottom_y = points
            .iter()
            .max_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;

        if bottom_x - top_x > 70 || bottom_y - top_y > 70 {
            continue;
        }

        return i;
    }
    0
}

#[test]
fn regex_test() {
    let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>";

    assert_eq!(
        generator(input),
        &[
            Point {
                id: 1,
                position: Position { x: 9, y: 1 },
                velocity: Velocity { dx: 0, dy: 2 }
            },
            Point {
                id: 2,
                position: Position { x: 7, y: 0 },
                velocity: Velocity { dx: -1, dy: 0 }
            },
            Point {
                id: 3,
                position: Position { x: 3, y: -2 },
                velocity: Velocity { dx: -1, dy: 1 }
            },
            Point {
                id: 4,
                position: Position { x: 6, y: 10 },
                velocity: Velocity { dx: -2, dy: -1 }
            }
        ]
    );
}
