use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

pub struct Claim {
    id: u32,
    left_edge: u32,
    top_edge: u32,
    width: u32,
    height: u32,
}

impl Claim {
    pub fn left(&self) -> u32 {
        self.left_edge
    }

    pub fn right(&self) -> u32 {
        self.left_edge + self.width
    }

    pub fn top(&self) -> u32 {
        self.top_edge
    }

    pub fn bottom(&self) -> u32 {
        self.top_edge + self.height
    }

    pub fn intersects(&self, other: &Claim) -> bool {
        !(self.left() > other.right()
            || self.right() < other.left()
            || self.top() > other.bottom()
            || self.bottom() < other.top())
    }
}

#[aoc_generator(day3)]
fn parse_claims(input: &str) -> Vec<Claim> {
    let regex = Regex::new(
        r"#(?P<id>\d+) @ (?P<left_edge>\d+),(?P<top_edge>\d+): (?P<width>\d+)x(?P<height>\d+)",
    )
    .unwrap();

    regex
        .captures_iter(input)
        .map(|mtch| Claim {
            id: mtch.name("id").unwrap().as_str().parse().unwrap(),
            left_edge: mtch.name("left_edge").unwrap().as_str().parse().unwrap(),
            top_edge: mtch.name("top_edge").unwrap().as_str().parse().unwrap(),
            width: mtch.name("width").unwrap().as_str().parse().unwrap(),
            height: mtch.name("height").unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn day_3_part_1(claims: &[Claim]) -> u32 {
    let mut two_or_more = 0;

    let is_inside = |claim: &Claim, x, y| {
        claim.top_edge < y
            && claim.top_edge + claim.height >= y
            && claim.left_edge < x
            && claim.left_edge + claim.width >= x
    };

    for (x, y) in (0..1000)
        .map(move |x| (0..1000).map(move |y| (x, y)))
        .flatten()
    {
        if claims.iter().filter(|claim| is_inside(claim, x, y)).count() >= 2 {
            two_or_more += 1;
        }
    }

    two_or_more
}

#[aoc(day3, part2)]
pub fn day_3_part_2(claims: &[Claim]) -> u32 {
    claims
        .par_iter()
        .find_any(|claim| {
            claims
                .par_iter()
                .filter(|claim2| claim2.id != claim.id)
                .all(|claim2| !claim.intersects(claim2))
        })
        .unwrap()
        .id
}
