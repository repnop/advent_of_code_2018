#![allow(dead_code)]

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
pub enum Polarity {
    Positive(char),
    Negative(char),
}

impl Polarity {
    pub fn new(c: char) -> Polarity {
        if c.is_uppercase() {
            Polarity::Positive(c.to_ascii_lowercase())
        } else {
            Polarity::Negative(c)
        }
    }

    #[cfg(test)]
    pub fn to_char(self) -> char {
        match self {
            Polarity::Positive(c) => c.to_ascii_uppercase(),
            Polarity::Negative(c) => c,
        }
    }

    pub fn cancels(self, other: Self) -> bool {
        match (self, other) {
            (Polarity::Positive(c1), Polarity::Negative(c2)) if c1 == c2 => true,
            (Polarity::Negative(c1), Polarity::Positive(c2)) if c1 == c2 => true,
            _ => false,
        }
    }
}

struct Polymer(u32, Vec<Option<Polarity>>);

impl From<&str> for Polymer {
    fn from(input: &str) -> Polymer {
        Polymer(
            input.len() as u32,
            input.chars().map(|c| Some(Polarity::new(c))).collect(),
        )
    }
}

impl std::fmt::Debug for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.1.iter().filter(|f| f.is_some()).collect::<Vec<_>>()
        )
    }
}

impl std::ops::Deref for Polymer {
    type Target = [Option<Polarity>];

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl std::ops::DerefMut for Polymer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl Polymer {
    fn next(&self, index: usize) -> Option<(usize, Polarity)> {
        self.1[index..]
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .map(|(i, inner)| (i + index, inner.unwrap()))
            .next()
    }

    fn len(&self) -> u32 {
        self.0
    }

    fn remove(&mut self, index: usize) {
        self[index] = None;
        self.0 -= 1;
    }
}

#[aoc(day5, part1, faster)]
pub fn faster(input: &[u8]) -> u32 {
    let mut top = 0;
    let mut v = Vec::with_capacity(input.len() / 3);

    for &n in input {
        if top ^ 0x20 == n {
            v.pop();
            top = *v.last().unwrap_or(&0);
        } else {
            top = n;
            v.push(n);
        }
    }

    v.len() as u32
}

#[aoc(day5, part2)]
pub fn day_5_part_2(input: &[u8]) -> u32 {
    (b'A'..=b'Z')
        .map(|c| {
            faster(
                &input
                    .iter()
                    .filter(|c2| *c2 & !0x20 != c && **c2 != c)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        .min()
        .unwrap()
}

#[aoc(day5, part1)]
pub fn basic_solution(input: &str) -> u32 {
    let mut polymer = Polymer::from(input.trim());
    let mut last_len = polymer.len();

    loop {
        let mut idx = 0;

        'inner: while idx <= last_len {
            let (idx1, current) = match polymer.next(idx as usize) {
                Some((idx, current)) => (idx, current),
                None => break 'inner,
            };

            let (idx2, next) = match polymer.next(idx1 + 1) {
                Some((idx, next)) => (idx, next),
                None => break 'inner,
            };

            if current.cancels(next) {
                polymer.remove(idx1);
                polymer.remove(idx2);
                idx = 0;
            } else {
                idx += 1;
            }
        }

        if last_len == polymer.len() {
            break;
        }

        last_len = polymer.len();
    }

    polymer.len()
}
