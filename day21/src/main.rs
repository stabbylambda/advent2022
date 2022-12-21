use std::collections::HashMap;

use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
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
    Literal(Option<i64>),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divides(&'a str, &'a str),
    Equals(&'a str, &'a str),
}

impl<'a> MonkeyValue<'a> {
    fn get_dependencies(&self) -> Option<(&'a str, &'a str)> {
        match self {
            MonkeyValue::Equals(l, r) => Some((*l, *r)),
            MonkeyValue::Plus(l, r) => Some((*l, *r)),
            MonkeyValue::Minus(l, r) => Some((*l, *r)),
            MonkeyValue::Times(l, r) => Some((*l, *r)),
            MonkeyValue::Divides(l, r) => Some((*l, *r)),
            MonkeyValue::Literal(_) => None,
        }
    }
}

type Input<'a> = Vec<(&'a str, MonkeyValue<'a>)>;

fn monkey_value<'a>(input: &'a str) -> IResult<&'a str, MonkeyValue<'a>> {
    alt((
        map(nom_i64, |x| MonkeyValue::Literal(Some(x))),
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

fn evaluate(current: &str, map: &HashMap<&str, MonkeyValue>) -> Option<i64> {
    let current = map[current];

    match current {
        MonkeyValue::Literal(x) => x,
        MonkeyValue::Plus(l, r) => evaluate(l, map).and_then(|l| evaluate(r, map).map(|r| l + r)),
        MonkeyValue::Minus(l, r) => evaluate(l, map).and_then(|l| evaluate(r, map).map(|r| l - r)),
        MonkeyValue::Times(l, r) => evaluate(l, map).and_then(|l| evaluate(r, map).map(|r| l * r)),
        MonkeyValue::Divides(l, r) => {
            evaluate(l, map).and_then(|l| evaluate(r, map).map(|r| l / r))
        }
        MonkeyValue::Equals(l, r) => {
            evaluate(l, map).and_then(|l| evaluate(r, map).map(|r| (l == r) as i64))
        }
    }
}

fn problem1(input: &Input) -> i64 {
    let map: HashMap<_, _> = input.into_iter().map(|x| *x).collect();
    evaluate("root", &map).unwrap()
}

fn solve(
    id: &str,
    map: &mut HashMap<&str, MonkeyValue>,
    value_to_solve: Option<i64>,
) -> Option<i64> {
    // We're here, so just return the value to solve, we've already solved it
    if id == "humn" {
        return value_to_solve;
    }

    let monkey = map[id];
    let Some((l, r)) = monkey.get_dependencies() else { panic!(); };

    // one of these will be none and the other will be a solved monkey
    let left = evaluate(l, &map);
    let right = evaluate(r, &map);

    let (unknown_id, known) = match (left, right) {
        (None, Some(x)) => (l, x),
        (Some(x), None) => (r, x),
        _ => panic!("No idea what went wrong here"),
    };

    // this is the starting point, so go ahead and recurse on the unknown side of the equation
    if id == "root" {
        return solve(unknown_id, map, Some(known));
    }

    let value_to_solve = value_to_solve.map(|value| match monkey {
        MonkeyValue::Plus(_, _) => value - known,
        MonkeyValue::Minus(_, _) if l == unknown_id => value + known,
        MonkeyValue::Minus(_, _) if r == unknown_id => known - value,
        MonkeyValue::Divides(_, _) if l == unknown_id => value * known,
        MonkeyValue::Divides(_, _) if r == unknown_id => known / value,
        MonkeyValue::Times(_, _) => value / known,

        _ => unreachable!(),
    });

    solve(unknown_id, map, value_to_solve)
}

fn problem2(input: &Input) -> i64 {
    let mut map: HashMap<_, _> = input.into_iter().map(|x| *x).collect();

    // set root to an Equals Monkey
    let root = map.get_mut("root").unwrap();
    let Some((l, r)) = root.get_dependencies() else { panic!()};
    *root = MonkeyValue::Equals(l, r);

    // set humn to None so the whole tree and any tree that contains it will eval to None
    let humn = map.get_mut("humn").unwrap();
    *humn = MonkeyValue::Literal(None);

    // keep solving "the other side" of the tree so we can get a literal to factor out of the other side of the equality check
    solve("root", &mut map, None).unwrap()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    #[ignore]
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
        assert_eq!(result, 301)
    }
}
