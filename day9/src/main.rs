use std::collections::{BTreeMap, BTreeSet};

use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Step = (Direction, u32);
type Input = Vec<Step>;
fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(anychar, tag(" "), nom_u32),
            |(dir, count)| match dir {
                'U' => (Direction::Up, count),
                'D' => (Direction::Down, count),
                'L' => (Direction::Left, count),
                'R' => (Direction::Right, count),
                _ => panic!(),
            },
        ),
    )(input);

    result.unwrap().1
}

fn is_adjacent(_head @ (x1, y1): (i32, i32), tail: (i32, i32)) -> bool {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if (x1 + dx, y1 + dy) == tail {
                return true;
            }
        }
    }

    false
}

fn problem1(input: &Input) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    visited.insert((0, 0));
    for (dir, count) in input {
        for _n in 1..=*count {
            match dir {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            if !is_adjacent(head, tail) {
                let (tx, ty) = tail;
                let delta = (head.0 - tail.0, head.1 - tail.1);

                tail = match delta {
                    // T west of H
                    (2, 1) => (tx + 1, ty + 1),
                    (2, 0) => (tx + 1, ty),
                    (2, -1) => (tx + 1, ty - 1),

                    // T south of H
                    (1, 2) => (tx + 1, ty + 1),
                    (0, 2) => (tx, ty + 1),
                    (-1, 2) => (tx - 1, ty + 1),

                    // T north of H
                    (1, -2) => (tx + 1, ty - 1),
                    (0, -2) => (tx, ty - 1),
                    (-1, -2) => (tx - 1, ty - 1),

                    // T east of H
                    (-2, 1) => (tx - 1, ty + 1),
                    (-2, 0) => (tx - 1, ty),
                    (-2, -1) => (tx - 1, ty - 1),

                    _ => panic!(),
                };

                visited.insert(tail.clone());
            }
        }
    }
    visited.len()
}

fn problem2(lines: &Input) -> u32 {
    0
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
        assert_eq!(result, 13)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 36)
    }
}
