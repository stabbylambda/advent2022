use core::time;

use ndarray::prelude::*;

use common::get_raw_input;

pub mod parser;

fn main() {
    let input = get_raw_input();
    let input = parser::parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Caves;

#[derive(Debug)]
struct Valve {
    id: usize,
    name: String,
    flow_rate: u32,
    neighbors: Vec<usize>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Caves {
    valves: Vec<Valve>,
    aa_index: usize,
}

fn floyd_warshall(valves: &[Valve]) -> Array2<u32> {
    // initialize with an arbitrarily large number that isn't u32::MAX because I don't want to overflow
    let mut dist = Array2::from_elem((valves.len(), valves.len()), 100_000);
    for (v, valve) in valves.iter().enumerate() {
        for &e in &valve.neighbors {
            dist[[v, e]] = 1;
        }
        dist[[v, v]] = 0;
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[[i, j]] = dist[[i, j]].min(dist[[i, k]] + dist[[k, j]])
            }
        }
    }

    dist
}

fn find_all_paths<'a>(
    distances: &'a Array2<u32>,
    non_zero_valves: &'a [&'a Valve],
    start: usize,
    opened_valves: Vec<&'a Valve>,
    time_left: u32,
) -> Vec<Vec<&'a Valve>> {
    let remaining: Vec<(&Valve, u32)> = non_zero_valves
        .iter()
        .filter_map(|next| {
            let dist = distances[[start, next.id]];
            // only find the non-opened valves that we have time to get to
            if !opened_valves.contains(&next) && dist < time_left {
                Some((*next, dist))
            } else {
                None
            }
        })
        .collect();

    if remaining.is_empty() {
        // we're at the end, so this is a valid path
        vec![opened_valves.clone()]
    } else {
        // otherwise we want to go over all the remaining paths
        remaining
            .iter()
            .flat_map(|(x, dist)| {
                // get the remaining time to move there and open a valve
                let new_time_left = time_left - dist - 1;
                let mut new_opened_valves = opened_valves.clone();
                new_opened_valves.push(*x);

                find_all_paths(
                    distances,
                    non_zero_valves,
                    x.id,
                    new_opened_valves,
                    new_time_left,
                )
            })
            .collect()
    }
}

fn path_score(path: &[&Valve], start: usize, distances: &Array2<u32>, time_left: u32) -> u32 {
    let (_, _, score) = path
        .iter()
        .fold((start, time_left, 0), |(pos, time_left, score), v| {
            let time_left = time_left - distances[[pos, v.id]] - 1;
            let score = score + v.flow_rate * time_left;

            (v.id, time_left, score)
        });

    let path_str: Vec<&str> = path.iter().map(|x| x.name.as_str()).collect();
    let path_str = path_str.join(" -> ");
    println!("{path_str} = {score}");

    score
}

fn problem1(caves: &Input) -> u32 {
    let distances = floyd_warshall(&caves.valves);
    let non_zero_valves: Vec<&Valve> = caves.valves.iter().filter(|x| x.flow_rate != 0).collect();

    find_all_paths(&distances, &non_zero_valves, caves.aa_index, Vec::new(), 30)
        .iter()
        .map(|path| path_score(path, caves.aa_index, &distances, 30))
        .max()
        .unwrap()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parser::parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 1651)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
