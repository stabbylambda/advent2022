use std::collections::HashMap;

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64 as nom_u64},
    combinator::map,
    multi::separated_list0,
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

#[derive(Debug, Clone, Copy)]
enum MonkeyValue<'a> {
    Literal(u64),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divides(&'a str, &'a str),
}

type Input<'a> = Vec<(&'a str, MonkeyValue<'a>)>;

fn monkey_value<'a>(input: &'a str) -> IResult<&'a str, MonkeyValue<'a>> {
    alt((
        map(nom_u64, |x| MonkeyValue::Literal(x)),
        map(separated_pair(alpha1, tag(" + "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Plus(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" - "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Minus(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" * "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Times(lhs, rhs)
        }),
        map(separated_pair(alpha1, tag(" / "), alpha1), |(lhs, rhs)| {
            MonkeyValue::Divides(lhs, rhs)
        }),
    ))(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> =
        separated_list0(newline, separated_pair(alpha1, tag(": "), monkey_value))(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u64 {
    let mut map: HashMap<_, _> = input.into_iter().map(|x| *x).collect();
    evaluate("root", &mut map)
}

fn evaluate(current: &str, map: &mut HashMap<&str, MonkeyValue>) -> u64 {
    let current = map[current];

    match current {
        MonkeyValue::Literal(x) => x,
        MonkeyValue::Plus(l, r) => evaluate(l, map) + evaluate(r, map),
        MonkeyValue::Minus(l, r) => evaluate(l, map) - evaluate(r, map),
        MonkeyValue::Times(l, r) => evaluate(l, map) * evaluate(r, map),
        MonkeyValue::Divides(l, r) => evaluate(l, map) / evaluate(r, map),
    }
}

fn problem2(_input: &Input) -> u64 {
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
        assert_eq!(result, 152)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
