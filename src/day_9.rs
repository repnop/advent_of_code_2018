#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, allow(unused_variables))]

use aoc_runner_derive::aoc;
use std::ptr::NonNull;

#[derive(Debug, Default)]
struct Circle {
    backing: Vec<u32>,
    current_pos: usize,
}

impl std::ops::Deref for Circle {
    type Target = Vec<u32>;

    fn deref(&self) -> &Vec<u32> {
        &self.backing
    }
}

impl std::ops::DerefMut for Circle {
    fn deref_mut(&mut self) -> &mut Vec<u32> {
        &mut self.backing
    }
}

#[aoc(day9, part1)]
pub fn day_9_part_1(input: &str) -> u32 {
    let players = input
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let last_marble_pts = input
        .split_whitespace()
        .nth(6)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut marble_count = last_marble_pts;
    let mut players: Vec<u32> = vec![0; players];
    let mut current_player = 0;
    let mut marble_nums = 1..;
    let mut circle = Circle::default();

    circle.push(0);

    while marble_count > 0 {
        let curr_num = marble_nums.next().unwrap();

        if curr_num % 23 == 0 {
            players[current_player] += curr_num;
            for _ in 0..6 {
                if circle.current_pos == 0 {
                    circle.current_pos = circle.len() - 1;
                } else {
                    circle.current_pos -= 1;
                }
            }
            let seven_ccw = circle.current_pos;
            players[current_player] += circle.remove(seven_ccw);
            circle.current_pos -= 1;
        } else {
            insert_clock_wise(&mut circle, 2, curr_num);
        }

        if current_player + 1 == players.len() {
            current_player = 0;
        } else {
            current_player += 1;
        }

        marble_count -= 1;
    }

    players.into_iter().max().unwrap()
}

fn insert_clock_wise(c: &mut Circle, positions_clockwise: usize, marble_num: u32) {
    let pos = (c.current_pos + positions_clockwise) % c.len();
    c.insert(pos + 1, marble_num);
    c.current_pos = pos;
}

#[derive(Debug, Default)]
struct Circle2 {
    backing: LinkedList,
    current_pos: NonNull<Node>,
}

impl std::ops::Deref for Circle2 {
    type Target = LinkedList;

    fn deref(&self) -> &LinkedList {
        &self.backing
    }
}

impl std::ops::DerefMut for Circle2 {
    fn deref_mut(&mut self) -> &mut LinkedList {
        &mut self.backing
    }
}

#[derive(Default)]
struct LinkedList {
    len: usize,
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
}

struct Node {
    prev: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
    value: u32,
}

impl LinkedList {
    fn new() -> LinkedList {
        Self::default()
    }

    fn push_back(&mut self, value: u32) -> NonNull<Node> {
        self.len += 1;
        let mut ptr = NonNull::new(Box::into_raw(Box::new(Node {
            prev: None,
            next: None,
            value,
        })))
        .unwrap();

        if self.tail.is_none() {
            self.tail = Some(ptr);
        } else {
            unsafe {
                ptr.as_mut().prev = self.tail;
                self.tail.unwrap().as_mut().next = Some(ptr);
            }
            self.tail = Some(ptr);
        }

        self.tail.unwrap()
    }

    fn push_front(&mut self, value: u32) -> NonNull<Node> {
        self.len += 1;
        let mut ptr = NonNull::new(Box::into_raw(Box::new(Node {
            prev: None,
            next: None,
            value,
        })))
        .unwrap();

        if self.head.is_none() {
            self.head = Some(ptr);
        } else {
            unsafe {
                ptr.as_mut().next = self.head;
                self.head.unwrap().as_mut().prev = Some(ptr);
            }
            self.head = Some(ptr);
        }

        self.head.unwrap()
    }

    fn push_after(&mut self, after: NonNull<Node>, value: u32) -> NonNull<Node> {
        unsafe {
            if after.as_mut().next.is_none() {
                self.push_back(value)
            } else if after.as_mut().prev.is_none() {
                self.push_front(value)
            } else {
                self.len += 1;
                let mut ptr = NonNull::new(Box::into_raw(Box::new(Node {
                    prev: after.as_mut().prev,
                    next: after.as_mut().next,
                    value,
                })))
                .unwrap();

                after.as_mut().prev.unwrap().as_mut().next = Some(ptr);
                after.as_mut().next.unwrap().as_mut().prev = Some(ptr);

                ptr
            }
        }
    }

    fn remove(&mut self, node: NonNull<Node>) -> Option<u32> {
        unsafe {
            if node.as_mut().prev.is_none() {
                self.pop_front()
            } else if node.as_mut().next.is_none() {
                self.pop_back()
            } else {
                let ptr = node.as_mut().next;
                let ptr2 = node.as_mut().prev;

                ptr.unwrap().as_mut().prev = ptr2;
                ptr2.unwrap().as_mut().next = ptr;
                let ret = node.as_mut().value;

                Box::from_raw(node.as_ptr());
                Some(ret)
            }
        }
    }

    fn pop_back(&mut self) -> Option<u32> {
        if self.tail.is_none() {
            if self.head.is_none() {
                None
            } else {
                let ptr = self.head.unwrap();
                let ret = ptr.as_mut().value;
                self.head = None;

                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        } else {
            self.tail.unwrap().as_mut().prev.unwrap().as_mut().next = None;
            let ptr = self.tail.unwrap();
            self.tail = self.tail.unwrap().as_mut().prev;

            let ret = ptr.as_mut().value;
            Box::from_raw(ptr.as_ptr());

            Some(ret)
        }
    }

    fn pop_front(&mut self) -> Option<u32> {
        if self.head.is_none() {
            if self.tail.is_none() {
                None
            } else {
                let ptr = self.tail.unwrap();
                let ret = ptr.as_mut().value;
                self.tail = None;

                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        } else {
            self.head.unwrap().as_mut().next.unwrap().as_mut().prev = None;
            let ptr = self.head.unwrap();
            self.head = self.head.unwrap().as_mut().next;

            let ret = ptr.as_mut().value;
            Box::from_raw(ptr.as_ptr());

            Some(ret)
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl std::ops::Drop for LinkedList {
    fn drop(&mut self) {
        while self.head != None {
            self.pop_front();
        }
    }
}

#[aoc(day9, part2)]
pub fn day_9_part_2(input: &str) -> u32 {
    let players = input
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let last_marble_pts = input
        .split_whitespace()
        .nth(6)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut marble_count = last_marble_pts * 100;
    let mut players: Vec<u32> = vec![0; players];
    let mut current_player = 0;
    let mut marble_nums = 1..;
    let mut circle = Circle2::default();

    circle.push_front(0);

    while marble_count > 0 {
        let curr_num = marble_nums.next().unwrap();

        if curr_num % 23 == 0 {
            players[current_player] += curr_num;
            for _ in 0..6 {
                if circle.current_pos == 0 {
                    circle.current_pos = circle.len() - 1;
                } else {
                    circle.current_pos -= 1;
                }
            }
            let seven_ccw = circle.current_pos;
            let mut foo = circle.split_off(seven_ccw);
            players[current_player] += foo.pop_back().unwrap();
            circle.append(&mut foo);
            circle.current_pos -= 1;
        } else {
            insert_clock_wise2(&mut circle, 2, curr_num);
        }

        if current_player + 1 == players.len() {
            current_player = 0;
        } else {
            current_player += 1;
        }

        marble_count -= 1;
    }

    players.into_iter().max().unwrap()
}

fn insert_clock_wise2(c: &mut Circle2, positions_clockwise: usize, marble_num: u32) {
    let pos = (c.current_pos + positions_clockwise) % c.len();
    let mut foo = c.split_off(pos + 1);
    foo.push_back(marble_num);
    c.append(&mut foo);
    c.current_pos = pos;
}

#[test]
fn day_9_test() {
    let hs = day_9_part_1("9 players; last marble is worth 25 points");
    assert_eq!(hs, 32);
    let hs = day_9_part_1("10 players; last marble is worth 1618 points");
    assert_eq!(hs, 8317);
    let hs = day_9_part_1("13 players; last marble is worth 7999 points");
    assert_eq!(hs, 146_373);
}

#[test]
fn day_9_test_2() {
    let hs = day_9_part_2("9 players; last marble is worth 25 points");
    assert_eq!(hs, 32);
    let hs = day_9_part_2("10 players; last marble is worth 1618 points");
    assert_eq!(hs, 8317);
    let hs = day_9_part_2("13 players; last marble is worth 7999 points");
    assert_eq!(hs, 146_373);
}
