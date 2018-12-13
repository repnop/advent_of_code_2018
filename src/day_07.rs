use aoc_runner_derive::aoc;
use binary_heap_plus::BinaryHeap;
use hashbrown::HashSet;
use petgraph::{
    graphmap::{DiGraphMap, GraphMap},
    prelude::Direction,
};

type Name = char;
type Graph = GraphMap<Name, (), petgraph::prelude::Directed>;

pub fn generator(input: &str) -> Graph {
    let mut graph = DiGraphMap::new();

    for line in input.lines() {
        let before = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        let after = line
            .split_whitespace()
            .nth(7)
            .unwrap()
            .chars()
            .next()
            .unwrap();

        graph.add_edge(before, after, ());
    }

    graph
}

#[aoc(day7, part1)]
pub fn day_7_part_1(input: &str) -> String {
    let mut steps = String::new();
    let graph = generator(input);
    let mut processed = HashSet::new();

    let mut start_nodes = graph
        .all_edges()
        .filter(|(before, _, _)| {
            graph
                .neighbors_directed(*before, Direction::Incoming)
                .next()
                .is_none()
        })
        .map(|(before, _, _)| before)
        .collect::<Vec<_>>();

    start_nodes.sort_unstable();
    start_nodes.dedup();

    let mut queue = BinaryHeap::new_min();
    queue.extend(start_nodes.into_iter());

    while !queue.is_empty() {
        let root = queue.pop().unwrap();

        if !processed.contains(&root) {
            steps.push(root);
            processed.insert(root);

            queue.extend(graph.neighbors(root).filter(|node| {
                graph
                    .neighbors_directed(*node, Direction::Incoming)
                    .all(|node| processed.contains(&node))
            }));
        }
    }

    steps
}

#[derive(Default, Clone, Copy)]
pub struct Worker {
    start_time: u32,
    current_time: u32,
    processing: Option<Name>,
}

impl Worker {
    pub fn give_work(&mut self, start_time: u32, n: Name) {
        self.start_time = start_time;
        self.current_time = start_time;
        self.processing = Some(n);
    }

    pub fn work(&mut self) -> Option<Name> {
        self.current_time += 1;

        if let Some(work) = self.processing {
            #[cfg(not(test))]
            let time = u32::from(work as u8 - 4);
            #[cfg(test)]
            let time = u32::from(work as u8 - 64);

            if self.current_time == self.start_time + time {
                let ret = self.processing;
                self.processing = None;
                ret
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn has_work(&self) -> bool {
        self.processing.is_some()
    }
}

#[aoc(day7, part2)]
pub fn day_7_part_2(input: &str) -> u32 {
    let graph = generator(input);
    let mut processed = HashSet::new();

    let mut start_nodes = graph
        .all_edges()
        .filter(|(before, _, _)| {
            graph
                .neighbors_directed(*before, Direction::Incoming)
                .next()
                .is_none()
        })
        .map(|(before, _, _)| before)
        .collect::<Vec<_>>();

    start_nodes.sort_unstable();
    start_nodes.dedup();

    let mut queue = BinaryHeap::new_min();
    queue.extend(start_nodes.into_iter());

    #[cfg(not(test))]
    let mut workers = [Worker::default(); 5];
    #[cfg(test)]
    let mut workers = [Worker::default(); 2];
    let mut time = 0;

    for worker in workers.iter_mut() {
        if let Some(node) = queue.pop() {
            worker.give_work(time, node);
        }
    }

    while workers.iter().any(|worker| worker.has_work()) || !queue.is_empty() {
        /*#[cfg(not(test))]
        {
            println!(
                "Time: {:<10}{}{:>10}{:>10}{:>10}{:>10}",
                time,
                workers[0].processing.unwrap_or('.'),
                workers[1].processing.unwrap_or('.'),
                workers[2].processing.unwrap_or('.'),
                workers[3].processing.unwrap_or('.'),
                workers[4].processing.unwrap_or('.'),
            );
        }*/

        for worker in workers.iter_mut() {
            if let Some(root) = worker.work() {
                if !processed.contains(&root) {
                    processed.insert(root);

                    queue.extend(graph.neighbors(root).filter(|node| {
                        graph
                            .neighbors_directed(*node, Direction::Incoming)
                            .all(|node| processed.contains(&node))
                    }));
                }
            }
        }

        for worker in workers.iter_mut() {
            if !worker.has_work() {
                if let Some(node) = queue.pop() {
                    worker.give_work(time, node);
                }
            }
        }

        time += 1;
    }

    time
}

#[test]
fn smol_test_7() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    assert_eq!(day_7_part_1(input), "CABDFE");
}

#[test]
fn smol_test_7_globi() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step B can begin.
Step A must be finished before step F can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    assert_eq!(day_7_part_1(input), "CABDFE");
}

#[test]
fn smol_test_7_part_2() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    assert_eq!(day_7_part_2(input), 15);
}
