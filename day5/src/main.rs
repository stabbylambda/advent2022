use common::get_input_strings;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha0, anychar, char, u32 as nom_u32};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
fn main() {
    let lines = get_input_strings();
    let mut input = parse(&lines);

    let score = problem1(&mut input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Stack<'a> = Vec<&'a str>;

#[derive(Debug)]
struct Move {
    count: u32,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input<'a> {
    stacks: Vec<Stack<'a>>,
    moves: Vec<Move>,
}

impl Input<'_> {
    fn execute(&mut self) -> String {
        for m in &self.moves {
            for _ in 0..m.count {
                let from_stack = self.stacks.get_mut(m.from - 1).unwrap();
                let Some(x) = from_stack.pop() else {
                    panic!();
                };
                let to_stack = self.stacks.get_mut(m.to - 1).unwrap();
                to_stack.push(x);
            }
        }

        self.stacks
            .iter()
            .map(|s| s.last().unwrap().to_owned())
            .collect()
    }
}

fn parse_crate(s: &str) -> IResult<&str, Option<&str>> {
    map(
        alt((delimited(char('['), alpha0, char(']')), tag("   "))),
        |s| {
            if s == "   " {
                None
            } else {
                Some(s)
            }
        },
    )(s)
}

fn parse_row(s: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list0(tag(" "), parse_crate)(s)
}

fn parse_starting(lines: &[String]) -> Vec<Stack> {
    let (_, rest) = lines.split_last().unwrap();
    let rows: Vec<Vec<Option<&str>>> = rest.iter().map(|l| parse_row(l).unwrap().1).collect();
    let stack_count = rows[0].len();
    let mut stacks = Vec::new();
    for n in 0..stack_count {
        let mut stack = Vec::new();
        for row in &rows[..] {
            if let Some(s) = row[n] {
                stack.push(s);
            };
        }
        stack.reverse();
        stacks.push(stack);
    }
    stacks
}

fn parse_move(s: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), nom_u32),
            preceded(tag(" from "), nom_u32),
            preceded(tag(" to "), nom_u32),
        )),
        |(count, from, to)| Move {
            count,
            from: from as usize,
            to: to as usize,
        },
    )(s)
}

fn parse_moves(input: &[String]) -> Vec<Move> {
    input.iter().map(|s| parse_move(s).unwrap().1).collect()
}

fn parse(lines: &[String]) -> Input {
    let mut input = lines.split(|l| l.is_empty());
    let stacks = input.next().unwrap();
    let stacks = parse_starting(stacks);

    let moves = input.next().unwrap();
    let moves = parse_moves(moves);

    Input { stacks, moves }
}

fn problem1(input: &mut Input) -> String {
    input.execute()
}

fn problem2(input: &Input) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{parse, parse_crate, parse_row, problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let mut input = parse(&lines);
        let result = problem1(&mut input);
        assert_eq!(result, "CMZ")
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let input = parse(&lines);
        let result = problem2(&input);
        assert_eq!(result, "")
    }
    #[test]
    fn testparse() {
        assert_eq!(parse_crate("[D]").unwrap().1, Some("D"));
        assert_eq!(parse_crate("   ").unwrap().1, None);

        let expected = vec![Some("Z"), Some("M"), Some("P")];
        assert_eq!(parse_row("[Z] [M] [P]").unwrap().1, expected);

        let expected = vec![None, Some("D"), None];
        assert_eq!(parse_row("    [D]    ").unwrap().1, expected);
    }
}
