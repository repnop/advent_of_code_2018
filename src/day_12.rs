use aoc_runner_derive::aoc;
use hashbrown::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantState {
    Absent,
    Present,
}

impl PlantState {
    fn from_char(c: char) -> PlantState {
        match c {
            '.' => PlantState::Absent,
            '#' => PlantState::Present,
            _ => panic!("Invalid character"),
        }
    }
}

impl std::cmp::PartialEq<PlantState> for (PlantState, i32) {
    fn eq(&self, other: &PlantState) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plants {
    state: Vec<(PlantState, i32)>,
    rules: Vec<([PlantState; 5], PlantState)>,
}

fn generator(input: &str) -> Plants {
    let chars = input.chars().skip(15);
    let mut state: Vec<_> = (-50..0).map(|i| (PlantState::Absent, i)).collect();
    let mut i = -1;
    state.extend(chars.take_while(|c| *c == '.' || *c == '#').map(|c| {
        i += 1;
        (
            if c == '.' {
                PlantState::Absent
            } else {
                PlantState::Present
            },
            i,
        )
    }));

    state.extend((i + 1..i + 50).map(|i| (PlantState::Absent, i)));

    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut pat = [PlantState::Absent; 5];
            let mut chars = line.chars();

            for pat in pat.iter_mut() {
                *pat = chars.next().map(PlantState::from_char).unwrap();
            }

            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();

            let to = PlantState::from_char(chars.next().unwrap());

            (pat, to)
        })
        .collect::<Vec<_>>();

    Plants { state, rules }
}

#[aoc(day12, part1)]
pub fn day_12_part_1(input: &str) -> i32 {
    let mut plants = generator(input);

    for _i in 0..20 {
        #[cfg(test)]
        {
            print!("{:>2}: ", _i);
            plants
                .state
                .iter()
                .filter(|(_, i)| *i >= -3 && *i <= 35)
                .for_each(|(s, _)| {
                    print!("{}", if *s == PlantState::Present { '#' } else { '.' });
                });
            println!();
        }

        let mut next_gen = plants.state.clone();

        for window in plants.state.windows(5) {
            'inner: for (pat, next_state) in &plants.rules {
                match window {
                    [_, _, mid, _, _] if window == pat => {
                        next_gen.iter_mut().find(|(_, id)| *id == mid.1).unwrap().0 = *next_state;

                        break 'inner;
                    }
                    [_, _, mid, _, _] => {
                        next_gen.iter_mut().find(|(_, id)| *id == mid.1).unwrap().0 =
                            PlantState::Absent;
                    }

                    _ => unreachable!(),
                }
            }
        }

        plants.state = next_gen;
    }

    plants
        .state
        .into_iter()
        .filter(|(s, _)| *s == PlantState::Present)
        .map(|(_, i)| i)
        .sum()
}

#[aoc(day12, part2)]
pub fn day_12_part_2(input: &str) -> i64 {
    let mut plants = generator(input);

    for _i in 0..200 {
        let mut next_gen = plants.state.clone();
        let lowest = next_gen[0].1;
        let highest = next_gen.last().unwrap().1;

        if next_gen
            .iter()
            .take(5)
            .any(|(s, _)| *s == PlantState::Present)
            || next_gen
                .iter()
                .rev()
                .take(5)
                .any(|(s, _)| *s == PlantState::Present)
        {
            next_gen.insert(0, (PlantState::Absent, lowest - 1));
            next_gen.insert(0, (PlantState::Absent, lowest - 2));
            next_gen.push((PlantState::Absent, highest + 1));
            next_gen.push((PlantState::Absent, highest + 2));
        }

        for window in plants.state.windows(5) {
            'inner: for (pat, next_state) in &plants.rules {
                match window {
                    [_, _, mid, _, _] if window == pat => {
                        next_gen.iter_mut().find(|(_, id)| *id == mid.1).unwrap().0 = *next_state;

                        break 'inner;
                    }
                    [_, _, mid, _, _] => {
                        next_gen.iter_mut().find(|(_, id)| *id == mid.1).unwrap().0 =
                            PlantState::Absent;
                    }

                    _ => unreachable!(),
                }
            }
        }

        plants.state = next_gen;
    }

    let row_offset = 200
        - plants
            .state
            .iter()
            .find(|(s, _)| *s == PlantState::Present)
            .unwrap()
            .1 as i64;

    let num_offset = plants
        .state
        .iter()
        .find(|(s, _)| *s == PlantState::Present)
        .unwrap()
        .1 as i64;

    let mut sum = 0;

    for i in plants
        .state
        .iter()
        .filter(|(s, _)| *s == PlantState::Present)
        .map(|(_, i)| i)
    {
        sum += 50_000_000_000 - row_offset + (*i as i64 - num_offset);
    }

    sum
}

#[test]
fn day_12_example() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    assert_eq!(day_12_part_1(input), 325);
}
