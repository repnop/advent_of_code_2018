#![allow(dead_code)]

use aoc_runner_derive::aoc;
use std::fmt::{self, Debug, Display};

#[cfg(not(test))]
const SIZE: usize = 150;
#[cfg(test)]
const SIZE: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Track {
    Vertical,
    Horizontal,
    Intersection,
    CurveUpDown,
    // |
    // \---
    //
    // ---\
    //    |
    //
    CurveLeftRight,
    //    |
    // ---/
    //
    // /---
    // |
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2 {
    y: i32,
    x: i32,
}

impl std::ops::Add<(i32, i32)> for Point2 {
    type Output = Self;
    fn add(self, other: (i32, i32)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl From<usize> for Point2 {
    fn from(u: usize) -> Self {
        Self {
            x: (u % SIZE) as i32,
            y: (u / SIZE) as i32,
        }
    }
}

impl From<Point2> for usize {
    fn from(p: Point2) -> usize {
        p.y as usize * SIZE + p.x as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntersectionState {
    Left,
    Straight,
    Right,
}

impl IntersectionState {
    pub fn next_state(&mut self) {
        use self::IntersectionState::*;

        *self = match self {
            Left => Straight,
            Straight => Right,
            Right => Left,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cart {
    id: i32,
    direction: Direction,
    intersection_state: IntersectionState,
    position: Point2,
}

pub struct Map {
    #[cfg(not(test))]
    tracks: Vec<Option<Track>>,
    #[cfg(test)]
    tracks: Vec<Option<Track>>,
    carts: Vec<Cart>,
}

impl Map {
    pub fn step(&mut self) -> Option<Point2> {
        self.carts.sort_unstable_by_key(|cart| cart.position);

        for i in 0..self.carts.len() {
            let cart = &mut self.carts[i];
            match cart.direction {
                Direction::Up => match self.tracks.get(usize::from(cart.position + (0, -1))) {
                    Some(Some(track)) => {
                        cart.position.y -= 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Left,
                            Track::CurveLeftRight => cart.direction = Direction::Right,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Left,
                                    Right => cart.direction = Direction::Right,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Down => match self.tracks.get(usize::from(cart.position + (0, 1))) {
                    Some(Some(track)) => {
                        cart.position.y += 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Right,
                            Track::CurveLeftRight => cart.direction = Direction::Left,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Right,
                                    Right => cart.direction = Direction::Left,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Left => match self.tracks.get(usize::from(cart.position + (-1, 0))) {
                    Some(Some(track)) => {
                        cart.position.x -= 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Up,
                            Track::CurveLeftRight => cart.direction = Direction::Down,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Down,
                                    Right => cart.direction = Direction::Up,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Right => match self.tracks.get(usize::from(cart.position + (1, 0))) {
                    Some(Some(track)) => {
                        cart.position.x += 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Down,
                            Track::CurveLeftRight => cart.direction = Direction::Up,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Up,
                                    Right => cart.direction = Direction::Down,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }

                    _ => unreachable!(),
                },
            }

            let cart = *cart;

            if let Some(pos) = self
                .carts
                .iter()
                .find(|cart2| cart.position == cart2.position && cart.id != cart2.id)
                .map(|cart| cart.position)
            {
                return Some(pos);
            }
        }

        None
    }

    pub fn step_part_2(&mut self) -> Option<Point2> {
        self.carts.sort_unstable_by_key(|cart| cart.position);

        if self.carts.len() == 1 {
            return Some(self.carts[0].position);
        }

        let mut already_processed = hashbrown::HashSet::new();

        while let Some(cart) = self
            .carts
            .iter_mut()
            .find(|cart| !already_processed.contains(&cart.id))
        {
            match cart.direction {
                Direction::Up => match self.tracks.get(usize::from(cart.position + (0, -1))) {
                    Some(Some(track)) => {
                        cart.position.y -= 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Left,
                            Track::CurveLeftRight => cart.direction = Direction::Right,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Left,
                                    Right => cart.direction = Direction::Right,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Down => match self.tracks.get(usize::from(cart.position + (0, 1))) {
                    Some(Some(track)) => {
                        cart.position.y += 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Right,
                            Track::CurveLeftRight => cart.direction = Direction::Left,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Right,
                                    Right => cart.direction = Direction::Left,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Left => match self.tracks.get(usize::from(cart.position + (-1, 0))) {
                    Some(Some(track)) => {
                        cart.position.x -= 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Up,
                            Track::CurveLeftRight => cart.direction = Direction::Down,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Down,
                                    Right => cart.direction = Direction::Up,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }
                    _ => unreachable!(),
                },
                Direction::Right => match self.tracks.get(usize::from(cart.position + (1, 0))) {
                    Some(Some(track)) => {
                        cart.position.x += 1;

                        match track {
                            Track::CurveUpDown => cart.direction = Direction::Down,
                            Track::CurveLeftRight => cart.direction = Direction::Up,
                            Track::Intersection => {
                                use self::IntersectionState::*;
                                match cart.intersection_state {
                                    Straight => {}
                                    Left => cart.direction = Direction::Up,
                                    Right => cart.direction = Direction::Down,
                                }
                                cart.intersection_state.next_state();
                            }
                            _ => {}
                        }
                    }

                    _ => unreachable!(),
                },
            }

            already_processed.insert(cart.id);

            let crashes = self
                .carts
                .iter()
                .filter_map(|cart| {
                    self.carts
                        .iter()
                        .find(|cart2| cart.position == cart2.position && cart.id != cart2.id)
                        .map(|cart| cart.id)
                })
                .collect::<Vec<_>>();

            self.carts = self
                .carts
                .iter()
                .filter(|cart| !crashes.contains(&cart.id))
                .cloned()
                .collect();
        }

        None
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            #[cfg(not(test))]
            tracks: vec![None; SIZE * SIZE],
            #[cfg(test)]
            tracks: vec![None; 7 * SIZE],
            carts: Default::default(),
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, track) in (&self.tracks[..]).iter().enumerate() {
            if let Some(track) = track {
                if let Some(cart) = self.carts.iter().find(|cart| cart.position == idx.into()) {
                    write!(
                        f,
                        "{}",
                        match cart.direction {
                            Direction::Up => "^",
                            Direction::Down => "v",
                            Direction::Left => "<",
                            Direction::Right => ">",
                        }
                    )?;
                } else {
                    write!(
                        f,
                        "{}",
                        match track {
                            Track::Vertical => "|",
                            Track::Horizontal => "-",
                            Track::Intersection => "+",
                            Track::CurveUpDown => "\\",
                            Track::CurveLeftRight => "/",
                        }
                    )?;
                }
            } else {
                write!(f, " ")?;
            }

            if idx % SIZE == SIZE - 1 {
                writeln!(f, "")?;
            }
        }

        writeln!(f, "")?;

        self.carts.iter().for_each(|cart| {
            writeln!(f, "{:?}", cart).unwrap();
        });

        Ok(())
    }
}

fn parse_map(input: &str) -> Map {
    let mut map = Map::default();
    let mut cart_ids = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim_right().chars().enumerate() {
            map.tracks[y * SIZE + x] = if c == ' ' {
                None
            } else {
                Some(match c {
                    '-' => Track::Horizontal,
                    '|' => Track::Vertical,
                    '\\' => Track::CurveUpDown,
                    '/' => Track::CurveLeftRight,
                    '+' => Track::Intersection,
                    'v' => {
                        map.carts.push(Cart {
                            id: cart_ids,
                            direction: Direction::Down,
                            intersection_state: IntersectionState::Left,
                            position: Point2 {
                                x: x as i32,
                                y: y as i32,
                            },
                        });
                        cart_ids += 1;
                        Track::Vertical
                    }
                    '^' => {
                        map.carts.push(Cart {
                            id: cart_ids,
                            direction: Direction::Up,
                            intersection_state: IntersectionState::Left,
                            position: Point2 {
                                x: x as i32,
                                y: y as i32,
                            },
                        });
                        cart_ids += 1;
                        Track::Vertical
                    }
                    '>' => {
                        map.carts.push(Cart {
                            id: cart_ids,
                            direction: Direction::Right,
                            intersection_state: IntersectionState::Left,
                            position: Point2 {
                                x: x as i32,
                                y: y as i32,
                            },
                        });
                        cart_ids += 1;
                        Track::Horizontal
                    }
                    '<' => {
                        map.carts.push(Cart {
                            id: cart_ids,
                            direction: Direction::Left,
                            intersection_state: IntersectionState::Left,
                            position: Point2 {
                                x: x as i32,
                                y: y as i32,
                            },
                        });
                        cart_ids += 1;
                        Track::Horizontal
                    }
                    _ => unreachable!(),
                })
            }
        }
    }

    map
}

#[aoc(day13, part1)]
pub fn day_13_part_1(input: &str) -> Point2 {
    let mut map = parse_map(input);
    loop {
        let ret = map.step();

        if let Some(ret) = ret {
            return ret;
        }
    }
}

#[aoc(day13, part2)]
pub fn day_13_part_2(input: &str) -> Point2 {
    let mut map = parse_map(input);
    loop {
        let ret = map.step_part_2();

        if let Some(ret) = ret {
            return ret;
        }
    }
}

#[test]
fn track_parse_test() {
    let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    let mut map = parse_map(input);

    println!("{:?}\n--------------------\n", map);

    for _ in 0..13 {
        map.step();
        println!("{:?}\n--------------------\n", map);
        std::thread::sleep(std::time::Duration::from_millis(750));
    }

    assert_eq!(map.step(), Some(Point2 { x: 7, y: 3 }));
}

#[test]
fn crash_test() {
    let input = r"/>-<\        
|   |        
| /<+-\      
| | | v      
\>+</ |      
  |   ^      
  \<->/      ";

    let mut map = parse_map(input);

    println!("{:?}\n--------------------\n", map);

    for _ in 0..4 {
        map.step_part_2();
        println!("{:?}\n--------------------\n", map);
        std::thread::sleep(std::time::Duration::from_millis(750));
    }

    assert_eq!(map.step_part_2(), Some(Point2 { x: 6, y: 4 }));
}
