use std::collections::HashMap;

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i32),
}
type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(tag("noop"), |_| Instruction::NoOp),
            map(preceded(tag("addx "), nom::character::complete::i32), |x| {
                Instruction::AddX(x)
            }),
        )),
    )(input);

    result.unwrap().1
}

fn problem1(lines: &Input) -> i32 {
    let mut cpu = CPU::new();
    cpu.execute(lines);

    cpu.signal
}

fn problem2(lines: &Input) -> u32 {
    todo!()
}

struct CPU {
    cycle: i32,
    register_x: i32,
    pipeline: HashMap<i32, Instruction>,
    signal: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            register_x: 1,
            cycle: 0,
            pipeline: HashMap::new(),
            signal: 0,
        }
    }

    fn increment_cycle(&mut self) {
        self.cycle += 1;

        let interesting = vec![20, 60, 100, 140, 180, 220];
        if interesting.contains(&self.cycle) {
            println!("Adding {} to signal", self.signal_strength());
            self.signal += self.signal_strength();
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for i in instructions {
            match i {
                Instruction::AddX(v) => {
                    self.increment_cycle();
                    self.increment_cycle();
                    self.register_x += v;
                }
                Instruction::NoOp => {
                    self.increment_cycle();
                }
            }
        }
    }

    fn signal_strength(&self) -> i32 {
        self.cycle * self.register_x
    }
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
        assert_eq!(result, 13140)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
