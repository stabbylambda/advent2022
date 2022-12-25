use common::get_raw_input;
use core::fmt::Debug;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
    fmt::Display,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Blizzard(Direction),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Blizzard(Direction::Up) => write!(f, "^"),
            Self::Blizzard(Direction::Down) => write!(f, "v"),
            Self::Blizzard(Direction::Left) => write!(f, "<"),
            Self::Blizzard(Direction::Right) => write!(f, ">"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Entity {
    coords: (u32, u32),
    tile: Tile,
}

type Point = (i64, i64, i64);
type Input = Vec<Vec<Tile>>;

fn print_map(input: &Input) {
    println!();
    for row in input {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
    println!();
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        many1(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('^'), |_| Tile::Blizzard(Direction::Up)),
            map(char('v'), |_| Tile::Blizzard(Direction::Down)),
            map(char('<'), |_| Tile::Blizzard(Direction::Left)),
            map(char('>'), |_| Tile::Blizzard(Direction::Right)),
            map(char('.'), |_| Tile::Empty),
        ))),
    )(input);
    result.unwrap().1
}

fn simulate(input: &Input, start: (usize, usize), end: (usize, usize), round: usize) -> usize {
    round
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();

    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    x
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

struct Valley {
    points: BTreeSet<Point>,
    height: i64,
    width: i64,
    cycle: i64,
}

impl Valley {
    fn is_end(&self, (x, y, z): (i64, i64, i64)) -> bool {
        x == self.width - 2 && y == self.height - 1
    }

    fn is_free(&self, p @ (x, y, z): &(i64, i64, i64)) -> bool {
        *x >= 0 && *y >= 0 && !self.points.contains(p)
    }

    fn new(input: &Input) -> Valley {
        let height = input.len() as i64;
        let width = input.first().unwrap().len() as i64;

        // the blizzards are periodic, so we only need to generate up to a certain size
        let cycle = lcm(height, width);

        let mut points: BTreeSet<Point> = BTreeSet::new();
        // we're going to create a 3D map where the third dimension is time

        for t in 0..cycle {
            // the start and end are always available
            points.insert((1, -1, t));
            points.insert((width - 2, height, t));

            for (y, row) in input.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let x = x as i64;
                    let y = y as i64;
                    match cell {
                        // walls are always walls
                        Tile::Wall => points.insert((x, y, t)),

                        // insert the positions of where the blizzards will be at all points in time
                        Tile::Blizzard(Direction::Up) => {
                            points.insert((x, (y - 1 - t).rem_euclid(height - 2) + 1, t))
                        }
                        Tile::Blizzard(Direction::Down) => {
                            points.insert((x, (y - 1 + t).rem_euclid(height - 2) + 1, t))
                        }
                        Tile::Blizzard(Direction::Left) => {
                            points.insert(((x - 1 - t).rem_euclid(width - 2) + 1, y, t))
                        }
                        Tile::Blizzard(Direction::Right) => {
                            points.insert(((x - 1 + t).rem_euclid(width - 2) + 1, y, t))
                        }

                        // we don't really need to do anything for the empty tiles
                        Tile::Empty => false,
                    };
                }
            }
        }

        Valley {
            points,
            height,
            width,
            cycle,
        }
    }
}

fn travel_time(valley: &Valley, start: (i64, i64, i64)) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();

    distances.insert(start, 0);
    queue.push((Reverse(0), start));

    while let Some((Reverse(distance), position @ (x, y, t))) = queue.pop() {
        // have we reached the end? if so, our third dimension is the travel time here
        if valley.is_end(position) {
            return Some(distance);
        }

        // get our neighbors in spacetime
        let next_time = (t + 1) % valley.cycle;
        let neighbors = [
            (x, y, next_time),     // wait
            (x - 1, y, next_time), // west
            (x + 1, y, next_time), // east
            (x, y - 1, next_time), // north
            (x, y + 1, next_time), // south
        ];

        for neighbor in neighbors {
            if valley.is_free(&neighbor) {
                let neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

                if *neighbor_distance > distance + 1 {
                    *neighbor_distance = distance + 1;
                    queue.push((Reverse(*neighbor_distance), neighbor));
                }
            }
        }
    }

    None
}

fn problem1(input: &Input) -> usize {
    let valley = Valley::new(input);
    travel_time(&valley, (1, 0, 0)).unwrap()
}

fn problem2(_input: &Input) -> u32 {
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
        assert_eq!(result, 18)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
