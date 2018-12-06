use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

type Name = u32;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    name: Option<Name>,
    x: i32,
    y: i32,
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut parts = line.split(", ");

            Point {
                name: Some(idx as u32),
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn day_6_part_1(input: &[Point]) -> usize {
    let mut grid = vec![Point::default(); 1000 * 1000];
    let mut manhattens = Vec::with_capacity(input.len());
    let mut areas = vec![0; input.len()];

    for i in 0..1_000_000 {
        let (x, y) = (i % 1000, i / 1000);
        let mut current_point = Point {
            name: None,
            x: x as i32,
            y: y as i32,
        };

        manhattens.extend(input.iter().map(|point| manhatten(current_point, *point)));
        manhattens.sort_unstable_by_key(|(_, p)| *p);

        let two_contest = manhattens[0].1 == manhattens[1].1;

        // TODO: optimize not to even need the grid

        grid[y * 1000 + x] = if two_contest {
            current_point
        } else {
            let (name, _) = manhattens[0];
            current_point.name = Some(name);
            areas[name as usize] += 1;

            current_point
        };

        manhattens.clear();
    }

    let mut disqualified = Vec::new();

    for y in 0..1000 {
        if y == 0 || y == 999 {
            for x in 0..1000 {
                match grid[y * 1000 + x].name {
                    Some(name) if !disqualified.contains(&name) => disqualified.push(name),
                    _ => {}
                }
            }
        } else {
            match grid[y * 1000].name {
                Some(name) if !disqualified.contains(&name) => disqualified.push(name),
                _ => {}
            }

            match grid[y * 1000 + 999].name {
                Some(name) if !disqualified.contains(&name) => disqualified.push(name),
                _ => {}
            }
        }
    }

    grid.par_iter()
        .filter(|p| p.name.is_some() && !disqualified.contains(&p.name.unwrap()))
        .map(|point| areas[point.name.unwrap() as usize])
        .max()
        .unwrap()
}

fn manhatten(p1: Point, p2: Point) -> (Name, i32) {
    (p2.name.unwrap(), (p1.x - p2.x).abs() + (p1.y - p2.y).abs())
}

#[test]
fn manhatten_test() {
    let p1 = Point {
        name: None,
        x: 10,
        y: 20,
    };
    let p2 = Point {
        name: Some(1),
        x: 10,
        y: 20,
    };

    assert_eq!(manhatten(p1, p2), (1, 0));
}

#[test]
fn smol_test() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    let points = generator(input);

    let mut grid = vec![Point::default(); 10 * 10];
    let mut manhattens = Vec::with_capacity(input.len());
    let mut areas = vec![0; points.len()];

    for i in 0..100 {
        let (x, y) = (i % 10, i / 10);

        let mut current_point = Point {
            name: None,
            x: x as i32,
            y: y as i32,
        };

        manhattens.extend(points.iter().map(|point| manhatten(current_point, *point)));
        manhattens.sort_unstable_by_key(|(_, p)| *p);

        let two_contest = manhattens[0].1 == manhattens[1].1;

        grid[y * 10 + x] = if two_contest {
            current_point
        } else {
            let (name, _) = manhattens[0];
            areas[name as usize] += 1;
            current_point.name = Some(name);

            current_point
        };

        println!("{} {:?}", y * 10 + x, grid[y * 10 + x]);

        /*print!(
            "{}",
            current_point
                .name
                .map(|n| ((n as u8 + b'A') as char).to_string())
                .unwrap_or_else(|| ".".to_string())
        );

        if x == 9 {
            println!();
        }*/

        manhattens.clear();
    }

    let mut disqualified = Vec::new();

    println!("(9, 9) = {:?}", grid[99]);

    for y in 0..10 {
        if y == 0 || y == 9 {
            for x in 0..10 {
                println!("{:?}", grid[y * 10 + x].name);
                if let Some(name) = grid[y * 10 + x].name {
                    println!("pushing {}", ((name as u8 + b'A') as char));
                    disqualified.push(name);
                }
            }
        } else {
            println!("{:?} {:?}", grid[y * 10].name, grid[y * 10 + 9].name);
            if let Some(name) = grid[y * 10].name {
                println!("pushing {}", ((name as u8 + b'A') as char));
                disqualified.push(name);
            }

            if let Some(name) = grid[y * 10 + 9].name {
                println!("pushing {}", ((name as u8 + b'A') as char));
                disqualified.push(name);
            }
        }
    }

    disqualified.dedup();

    assert_eq!(
        points
            .iter()
            .filter(|p| !disqualified.contains(&p.name.unwrap()))
            .map(|point| areas[point.name.unwrap() as usize])
            .max()
            .unwrap(),
        17
    );
}
