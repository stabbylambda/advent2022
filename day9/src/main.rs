use std::collections::BTreeSet;

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

    let score = problem2(&input, false);
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

fn problem(points: &mut Vec<(i32, i32)>, input: &Input, print: bool) -> usize {
    let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    visited.insert((0, 0));
    if print {
        println!("== Initial ==");
        display(points.to_vec(), 20);
    }
    for (dir, count) in input {
        if print {
            println!("== {dir:?} {count} ==");
        }
        for _n in 1..=*count {
            let (hx, hy) = points[0];
            // first move the leader
            let head = match dir {
                Direction::Up => (hx, hy + 1),
                Direction::Down => (hx, hy - 1),
                Direction::Left => (hx - 1, hy),
                Direction::Right => (hx + 1, hy),
            };

            points[0] = head;

            for k in 0..points.len() - 1 {
                let (hx, hy) = points[k];
                let (tx, ty) = points[k + 1];
                let (dx, dy) = (hx - tx, hy - ty);

                // then move the follower, who only needs to move if the leader is 2 spaces away
                let follower = match (dx, dy) {
                    // for problem 2, pure diagonal moves are possible
                    (2, 2) => (tx + 1, ty + 1),
                    (-2, -2) => (tx - 1, ty - 1),
                    (2, -2) => (tx + 1, ty - 1),
                    (-2, 2) => (tx - 1, ty + 1),

                    (2, _) => (tx + 1, ty + dy),
                    (-2, _) => (tx - 1, ty + dy),
                    (_, 2) => (tx + dx, ty + 1),
                    (_, -2) => (tx + dx, ty - 1),
                    _ => (tx, ty),
                };

                points[k + 1] = follower;
                // only track the tail
                if k + 1 == points.len() - 1 {
                    visited.insert(follower.clone());
                }
            }
            if print {
                display(points.to_vec(), 40);
            }
        }
    }
    visited.len()
}

fn problem1(input: &Input) -> usize {
    let mut points: Vec<(i32, i32)> = vec![(0, 0); 2];
    problem(&mut points, input, false)
}

fn display(positions: Vec<(i32, i32)>, size: i32) {
    let (ox, oy) = (size / 2, size / 2);
    let mut grid = vec![vec![None; size as usize]; size as usize];
    for (idx, (x, y)) in positions.iter().enumerate() {
        let cell = grid[(oy + *y) as usize][(ox + *x) as usize];
        grid[(oy + *y) as usize][(ox + *x) as usize] = match cell {
            None => Some(idx),
            Some(idx) => Some(idx),
        }
    }

    for y in (0usize..size as usize).rev() {
        for x in 0usize..size as usize {
            match grid[y][x] {
                Some(n) if n == 0 => print!("H"),
                Some(n) if n == positions.len() - 1 => print!("T"),
                Some(n) => print!("{n}"),
                None => print!("."),
            }
        }
        println!()
    }
    println!()
}

fn problem2(input: &Input, print: bool) -> usize {
    let mut points: Vec<(i32, i32)> = vec![(0, 0); 10];
    problem(&mut points, input, print)
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
    fn second1() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input, true);
        assert_eq!(result, 1)
    }

    #[test]
    fn second2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let input = parse(&input);
        let result = problem2(&input, true);
        assert_eq!(result, 36)
    }
}
