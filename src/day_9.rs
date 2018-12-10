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

#[derive(Default)]
struct Circle2 {
    backing: LinkedList,
    current_pos: Option<NonNull<Node>>,
}

impl std::fmt::Debug for Circle2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut curr_node = self.head;

        write!(f, "[")?;
        while curr_node.is_some() {
            unsafe {
                if curr_node == self.current_pos {
                    write!(f, "({}), ", curr_node.unwrap().as_ref().value)?;
                } else {
                    write!(f, "{}, ", curr_node.unwrap().as_ref().value)?;
                }

                curr_node = curr_node.unwrap().as_ref().next;
            }
        }
        write!(f, "]")?;

        Ok(())
    }
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

// Through me you pass into the city of woe: 
// Through me you pass into eternal pain: 
// Through me among the people lost for aye. 
// Justice the founder of my fabric moved: 
// To rear me was the task of Power divine,        
// Supremest Wisdom, and primeval Love.
// Before me things create were none, save things 
// Eternal, and eternal I endure. 
// Abandon all hope, ye who enter here.

#[derive(Default)]
struct LinkedList {
    len: usize,
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
}

#[derive(Debug)]
struct Node {
    prev: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
    value: u32,
}

impl LinkedList {
    #[allow(dead_code)]
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
            unsafe {
                self.head.unwrap().as_mut().next = self.tail;
            }
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

    fn push_after(&mut self, mut after: NonNull<Node>, value: u32) -> NonNull<Node> {
        unsafe {
            if after.as_ref().next.is_none() {
                self.push_back(value);
            } else {
                self.len += 1;
                let ptr = NonNull::new(Box::into_raw(Box::new(Node {
                    prev: Some(after),
                    next: after.as_ref().next,
                    value,
                })))
                .unwrap();

                if after.as_ref().next.is_some() {
                    after.as_ref().next.unwrap().as_mut().prev = Some(ptr);
                }
                after.as_mut().next = Some(ptr);

                //ptr
            }
        }

        after
    }

    #[allow(dead_code)]
    fn push_before(&mut self, mut after: NonNull<Node>, value: u32) -> NonNull<Node> {
        unsafe {
            if after.as_ref().next.is_none() {
                self.push_back(value)
            } else if after.as_ref().prev.is_none() {
                self.push_front(value)
            } else {
                self.len += 1;
                let ptr = NonNull::new(Box::into_raw(Box::new(Node {
                    prev: after.as_ref().prev,
                    next: Some(after),
                    value,
                })))
                .unwrap();

                after.as_ref().prev.unwrap().as_mut().next = Some(ptr);
                after.as_mut().prev = Some(ptr);

                ptr
            }
        }
    }

    fn remove(&mut self, node: NonNull<Node>) -> Option<u32> {
        unsafe {
            if node.as_ref().prev.is_none() {
                self.pop_front()
            } else if node.as_ref().next.is_none() {
                self.pop_back()
            } else {
                let ptr = node.as_ref().next;
                let ptr2 = node.as_ref().prev;

                ptr.unwrap().as_mut().prev = ptr2;
                ptr2.unwrap().as_mut().next = ptr;
                let ret = node.as_ref().value;

                Box::from_raw(node.as_ptr());
                Some(ret)
            }
        }
    }

    fn pop_back(&mut self) -> Option<u32> {
        if self.tail.is_none() {
            unsafe {
                let ptr = self.head?;
                let ret = ptr.as_ref().value;
                self.head = None;

                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        } else {
            unsafe {
                let mut tail_prev = self.tail.unwrap().as_mut().prev?;
                tail_prev.as_mut().next = None;
                let ptr = self.tail.unwrap();
                self.tail = self.tail.unwrap().as_mut().prev;

                let ret = ptr.as_ref().value;
                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        }
    }

    fn pop_front(&mut self) -> Option<u32> {
        if self.head.is_none() {
            unsafe {
                let ptr = self.tail?;
                let ret = ptr.as_ref().value;
                self.tail = None;

                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        } else {
            unsafe {
                let ptr = self.head.unwrap();

                if let Some(mut head_next) = self.head.unwrap().as_ref().next {
                    head_next.as_mut().prev = None;
                    self.head = self.head.unwrap().as_ref().next;
                }

                let ret = ptr.as_ref().value;
                Box::from_raw(ptr.as_ptr());

                Some(ret)
            }
        }
    }

    #[allow(dead_code)]
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
    circle.current_pos = circle.head;

    while marble_count > 0 {
        let curr_num = marble_nums.next().unwrap();

        if curr_num % 23 == 0 {
            players[current_player] += curr_num;
            for _ in 0..6 {
                unsafe {
                    if circle.current_pos.unwrap().as_ref().prev.is_none() {
                        circle.current_pos = circle.tail;
                    } else {
                        circle.current_pos = circle.current_pos.unwrap().as_ref().prev;
                    }
                }
            }
            let seven_ccw = circle.current_pos;
            unsafe {
                circle.current_pos = circle.current_pos.unwrap().as_ref().prev;
            }
            players[current_player] += circle.remove(seven_ccw.unwrap()).unwrap();
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

fn insert_clock_wise2(circle: &mut Circle2, positions_clockwise: usize, marble_num: u32) {
    for _ in 0..positions_clockwise {
        unsafe {
            if circle.current_pos.unwrap().as_ref().next.is_none() {
                circle.current_pos = circle.head;
            } else {
                circle.current_pos = circle.current_pos.unwrap().as_ref().next;
            }
        }
    }

    let node = circle.current_pos.unwrap();

    circle.current_pos = Some(circle.push_after(node, marble_num));
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
