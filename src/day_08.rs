use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn day_8_part_1(input: &str) -> u32 {
    let mut iter = input.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    helper_part_1(&mut iter)
}

pub fn helper_part_1(input: &mut impl Iterator<Item = u8>) -> u32 {
    let mut sum = 0;
    let children = if let Some(input) = input.next() {
        input
    } else {
        return 0;
    };

    let metadata = input.next().unwrap();

    for _ in 0..children {
        sum += helper_part_1(input);
    }

    for _ in 0..metadata {
        sum += u32::from(input.next().unwrap());
    }

    sum
}

#[aoc(day8, part2)]
pub fn day_8_part_2(input: &str) -> u32 {
    let mut iter = input.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    helper_part_2(&mut iter)
}

pub fn helper_part_2(input: &mut impl Iterator<Item = u8>) -> u32 {
    let mut sum = 0;
    let children = if let Some(input) = input.next() {
        input
    } else {
        return 0;
    };

    let metadata = input.next().unwrap();
    let mut child_sum = Vec::with_capacity(children as usize);

    for _ in 0..children {
        child_sum.push(helper_part_2(input));
    }

    if children == 0 {
        for _ in 0..metadata {
            sum += u32::from(input.next().unwrap());
        }
    } else {
        for _ in 0..metadata {
            sum += child_sum
                .get(input.next().unwrap() as usize - 1)
                .cloned()
                .unwrap_or(0);
        }
    }

    sum
}

#[test]
fn smol_test_8() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    assert_eq!(day_8_part_1(input), 138);
}

#[test]
fn smol_test_8_part_2() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    assert_eq!(day_8_part_2(input), 66);
}
