use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub year: u32,
    pub month: u32,
    pub time: Time,
    pub guard_id: Option<u32>,
    pub action: Action,
}

impl std::ops::Deref for Entry {
    type Target = Time;
    fn deref(&self) -> &Time {
        &self.time
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, Default, Eq, Hash, Clone, Copy)]
pub struct Time {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

impl Time {
    pub fn total(&self) -> u32 {
        self.day * 24 + self.hour * 60 + self.minute
    }
}

#[aoc_generator(day4)]
pub fn get_entries(input: &str) -> Vec<Entry> {
    //let mut regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] (Guard #\d+ begins shift|falls asleep|wakes up)").unwrap();
    let lines = input.lines();
    let mut entries = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();

        let mut date = parts.next().unwrap()[1..].split('-');
        let year = date.next().unwrap().parse().unwrap();
        let month = date.next().unwrap().parse().unwrap();
        let day = date.next().unwrap().parse().unwrap();

        let mut time = parts.next().unwrap().split(':');
        let hour = time.next().unwrap().parse().unwrap();
        let minute = time.next().unwrap();
        let minute = minute[0..minute.len() - 1].parse().unwrap();

        let action = parts.next().unwrap();
        let mut guard_id = None;

        let action = match action {
            "falls" => Action::FallsAsleep,
            "wakes" => Action::WakesUp,
            "Guard" => {
                let num: &str = parts.next().unwrap();
                guard_id = Some(num[1..].parse::<u32>().unwrap());
                Action::BeginsShift
            }
            _ => unreachable!(),
        };

        entries.push(Entry {
            year,
            month,
            time: Time { day, hour, minute },
            guard_id,
            action,
        });
    }

    entries.sort_unstable_by(|a, b| {
        use std::cmp::Ordering::*;

        match a.year.cmp(&b.year) {
            Equal => match a.month.cmp(&b.month) {
                Equal => match a.day.cmp(&b.day) {
                    Equal => match a.hour.cmp(&b.hour) {
                        Equal => a.minute.cmp(&b.minute),
                        order => order,
                    },
                    order => order,
                },
                order => order,
            },
            order => order,
        }
    });

    entries
}

fn common_part(input: &[Entry]) -> (HashMap<(u32, u32), u32>, HashMap<u32, u32>) {
    let mut tracker = HashMap::with_capacity(input.len());
    let mut mins_asleep = HashMap::with_capacity(input.len());
    let mut idx = 0;

    while idx < input.len() {
        let guard_id = input[idx].guard_id.unwrap();
        idx += 1;

        let mut last_asleep = Time::default();
        while idx < input.len() && input[idx].action != Action::BeginsShift {
            match input[idx].action {
                Action::FallsAsleep => {
                    last_asleep = input[idx].time;
                }

                Action::WakesUp => {
                    let wakes_up = input[idx].time;

                    *mins_asleep.entry(guard_id).or_insert(0) +=
                        wakes_up.total() - last_asleep.total();

                    for i in last_asleep.minute..wakes_up.minute {
                        *tracker.entry((guard_id, i)).or_insert(0) += 1;
                    }
                }
                _ => unreachable!(),
            }

            idx += 1;
        }
    }

    (tracker, mins_asleep)
}

#[aoc(day4, part1)]
pub fn day_4_part_1(input: &[Entry]) -> u32 {
    let (tracker, mins_asleep) = common_part(input);

    let (guard_id, _) = mins_asleep.iter().max_by_key(|x| x.1).unwrap();
    let minute = (*tracker
        .iter()
        .filter(|(k, _)| k.0 == *guard_id)
        .max_by_key(|(_, v)| **v)
        .unwrap()
        .0)
        .1;

    guard_id * minute
}

#[aoc(day4, part2)]
pub fn day_4_part_2(input: &[Entry]) -> u32 {
    let (tracker, _) = common_part(input);

    let ((guard_id, min), _) = tracker.iter().max_by_key(|(_, v)| **v).unwrap();

    guard_id * min
}

#[test]
fn parse_test() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift";
    let res = get_entries(input);
    assert_eq!(res[0].year, 1518);
    assert_eq!(res[0].month, 11);
    assert_eq!(res[0].day, 1);
    assert_eq!(res[0].hour, 0);
    assert_eq!(res[0].minute, 0);
    assert_eq!(res[0].guard_id, Some(10));
    assert_eq!(res[0].action, Action::BeginsShift);

    let input = "[1518-11-01 00:05] falls asleep";
    let res = get_entries(input);
    assert_eq!(res[0].year, 1518);
    assert_eq!(res[0].month, 11);
    assert_eq!(res[0].day, 1);
    assert_eq!(res[0].hour, 0);
    assert_eq!(res[0].minute, 5);
    assert_eq!(res[0].guard_id, None);
    assert_eq!(res[0].action, Action::FallsAsleep);

    let input = "[1518-11-01 00:25] wakes up";
    let res = get_entries(input);
    assert_eq!(res[0].year, 1518);
    assert_eq!(res[0].month, 11);
    assert_eq!(res[0].day, 1);
    assert_eq!(res[0].hour, 0);
    assert_eq!(res[0].minute, 25);
    assert_eq!(res[0].guard_id, None);
    assert_eq!(res[0].action, Action::WakesUp);
}

#[test]
fn sort_test() {
    let inputs = r"[1518-11-01 00:05] Guard #10 begins shift
[1518-11-01 00:02] Guard #10 begins shift
[1518-11-01 00:01] Guard #10 begins shift
[1518-11-01 00:04] Guard #10 begins shift
[1518-11-01 00:03] Guard #10 begins shift
[1518-11-01 00:06] Guard #10 begins shift";

    let entries = get_entries(inputs);

    for i in 1u32..=6 {
        assert_eq!(entries[i as usize - 1].minute, i);
    }
}
