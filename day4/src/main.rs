use common::get_input_strings;
use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair},
    IResult,
};
fn main() {
    let lines = get_input_strings();

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

type Range = (u32, u32);
#[derive(Debug)]
struct Assignment {
    first: Range,
    second: Range,
}
impl Assignment {
    fn new((first, second): (Range, Range)) -> Assignment {
        Assignment { first, second }
    }

    fn is_full_overlap(&self) -> bool {
        let (s1, e1) = self.first;
        let (s2, e2) = self.second;

        let contains12 = s1 <= s2 && e2 <= e1;
        let contains21 = s2 <= s1 && e1 <= e2;

        contains12 || contains21
    }
}

fn number(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |x| u32::from_str_radix(x, 10))(s)
}

fn get_assignments(s: &str) -> Assignment {
    map(
        separated_pair(
            separated_pair(number, char('-'), number),
            char(','),
            separated_pair(number, char('-'), number),
        ),
        Assignment::new,
    )(s)
    .unwrap()
    .1
}

fn problem1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|s| get_assignments(s))
        .filter(|x| x.is_full_overlap())
        .count() as u32
}

fn problem2(lines: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let result = problem1(&lines);
        assert_eq!(result, 2)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let result = problem2(&lines);
        assert_eq!(result, 0)
    }
}
