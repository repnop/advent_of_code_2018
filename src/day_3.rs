use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use regex::Regex;

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
            id: mtch[1].parse().unwrap(),
            left_edge: mtch[2].parse().unwrap(),
            top_edge: mtch[3].parse().unwrap(),
            width: mtch[4].parse().unwrap(),
            height: mtch[5].parse().unwrap(),
        })
        .collect()
}

#[aoc(day3, part1, safe)]
pub fn day_3_part_1(claims: &[Claim]) -> u32 {
    let mut inches = [0u8; 1000 * 1000];

    for claim in claims {
        for x in claim.left()..claim.right() {
            for y in claim.top()..claim.bottom() {
                let (x, y) = (x as usize, y as usize);
                inches[y * 1000 + x] += 1;
            }
        }
    }

    inches.iter().filter(|i| **i >= 2).count() as u32
}

#[aoc(day3, part1, totally_unsafe)]
pub fn day_3_part_1_totally_unsafe(claims: &[Claim]) -> u32 {
    let mut inches = [0u8; 1000 * 1000];

    #[repr(transparent)]
    struct TotallyUnsafe(*mut u8);

    unsafe impl Sync for TotallyUnsafe {}
    unsafe impl Send for TotallyUnsafe {}
    impl std::ops::Deref for TotallyUnsafe {
        type Target = *mut u8;
        fn deref(&self) -> &*mut u8 {
            &self.0
        }
    }

    let ptr = TotallyUnsafe(inches.as_mut_ptr());

    claims.par_iter().for_each(|claim| {
        (claim.top()..claim.bottom()).into_par_iter().for_each(|y| {
            for x in claim.left()..claim.right() {
                let (x, y) = (x as isize, y as isize);
                unsafe {
                    *ptr.offset(y * 1000 + x) += 1;
                }
            }
        });
    });

    inches.iter().filter(|i| **i >= 2).count() as u32
}

#[aoc(day3, part2, with_rayon)]
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

#[aoc(day3, part2, without_rayon)]
pub fn day_3_part_2_no_rayon(claims: &[Claim]) -> u32 {
    claims
        .iter()
        .find(|claim| {
            claims
                .iter()
                .filter(|claim2| claim2.id != claim.id)
                .all(|claim2| !claim.intersects(claim2))
        })
        .unwrap()
        .id
}
