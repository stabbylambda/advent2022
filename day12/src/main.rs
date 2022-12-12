use std::{cmp::Ordering, collections::BinaryHeap};

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, none_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

pub mod map;
use map::{CoordExt, Map};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
enum Position {
    Start,
    End,
    Normal(char),
}

impl Position {
    fn can_travel_to(&self, dest: &Position) -> bool {
        match (self, dest) {
            (Position::Start, Position::Normal(c)) => *c == 'a' || *c == 'b',
            (Position::Normal(c), Position::End) => *c == 'y' || *c == 'z',
            (Position::Normal(c1), Position::Normal(c2)) => {
                let dest_height = *c2 as u32;
                let start_height = *c1 as u32;

                dest_height <= start_height + 1
            }
            _ => false,
        }
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<Vec<Position>>> = separated_list1(
        newline,
        many1(alt((
            map(tag("S"), |_| Position::Start),
            map(tag("E"), |_| Position::End),
            map(none_of("\n"), |c| Position::Normal(c)),
        ))),
    )(input);

    let points = result.unwrap().1;
    let map = Map::new(points);

    Input::new(map)
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}
#[derive(Debug)]
struct Input {
    start: usize,
    finish: usize,
    map: Map<Position>,
}

impl Input {
    fn new(map: Map<Position>) -> Input {
        let width = map.width;
        let height = map.height;

        let mut start: usize = 0;
        let mut finish: usize = 0;
        for x in 0..width {
            for y in 0..height {
                match map.get((x, y)) {
                    Position::Start => start = (x, y).to_position(width),
                    Position::End => finish = (x, y).to_position(width),
                    Position::Normal(_) => {}
                }
            }
        }

        Input { map, start, finish }
    }
}

trait MapExt {
    fn to_edges(&self) -> Vec<Vec<Edge>>;
}

impl MapExt for Map<Position> {
    fn to_edges(&self) -> Vec<Vec<Edge>> {
        let mut edges: Vec<Vec<Edge>> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = (x, y);
                let letter = self.get(coord);
                edges.push(
                    self.neighbors(coord)
                        .iter()
                        .filter_map(|c| {
                            let neighbor = self.get(*c);
                            let valid_edge = letter.can_travel_to(neighbor);

                            if valid_edge {
                                // all edges are 1 for this purpose because we already eliminated all the
                                // invalid edges
                                Some(Edge {
                                    node: c.to_position(self.width),
                                    cost: 1,
                                })
                            } else {
                                None
                            }
                        })
                        .collect(),
                );
            }
        }
        edges
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn problem1(input: &Input) -> usize {
    let edges = input.map.to_edges();
    shortest_path(&edges, input.start, input.finish).unwrap()
}

fn problem2(input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 31)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
