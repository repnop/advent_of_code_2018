use aoc_runner_derive::aoc;
use std::collections::VecDeque;

#[aoc(day14, part1)]
fn day_14_part_1(input: &str) -> String {
    let mut recipes: VecDeque<_> = vec![3, 7].into();
    let mut elf_1 = 0;
    let mut elf_2 = 1;
    let limit: usize = input.parse().unwrap();

    while recipes.len() <= limit + 10 {
        let e1v = recipes[elf_1];
        let e2v = recipes[elf_2];
        let sum = e1v + e2v;

        if sum >= 10 {
            let tens = sum / 10;
            let ones = sum % 10;

            recipes.push_back(tens);
            recipes.push_back(ones);
        } else {
            recipes.push_back(sum);
        }

        elf_1 = (elf_1 + e1v + 1) % recipes.len();
        elf_2 = (elf_2 + e2v + 1) % recipes.len();
    }

    recipes
        .iter()
        .skip(limit)
        .take(10)
        .map(|n| n.to_string())
        .collect()
}

#[aoc(day14, part2)]
fn day_14_part_2(input: &str) -> usize {
    let mut recipes: VecDeque<_> = vec![3, 7].into();
    let mut elf_1 = 0;
    let mut elf_2 = 1;
    let limit: usize = input.parse().unwrap();
    let pat = input
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<_>>();

    while recipes.len() <= limit * 1000 {
        let e1v = recipes[elf_1];
        let e2v = recipes[elf_2];
        let sum = e1v + e2v;

        if sum >= 10 {
            let tens = sum / 10;
            let ones = sum % 10;

            recipes.push_back(tens);
            recipes.push_back(ones);
        } else {
            recipes.push_back(sum);
        }

        elf_1 = (elf_1 + e1v + 1) % recipes.len();
        elf_2 = (elf_2 + e2v + 1) % recipes.len();
    }

    let recipes: Vec<_> = recipes.into();

    recipes
        .windows(input.len())
        .enumerate()
        .find(|(_, window)| window == &pat.as_slice())
        .map(|(idx, _)| idx)
        .unwrap()
}

#[test]
fn day_14_test() {
    assert_eq!(day_14_part_1("9"), "5158916779");
    assert_eq!(day_14_part_1("5"), "0124515891");
    assert_eq!(day_14_part_1("18"), "9251071085");
    assert_eq!(day_14_part_1("2018"), "5941429882");

    assert_eq!(day_14_part_2("51589"), 9);
    assert_eq!(day_14_part_2("01245"), 5);
    assert_eq!(day_14_part_2("92510"), 18);
    assert_eq!(day_14_part_2("59414"), 2018);
}
