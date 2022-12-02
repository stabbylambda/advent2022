use std::cmp::Reverse;

use common::get_raw_input;
use nom::{
    branch::alt,
    character::complete::{digit1, line_ending},
    combinator::{eof, map_res},
    multi::{fold_many0, separated_list1},
    sequence::terminated,
    IResult,
};
fn main() {
    let lines = get_raw_input();
    let calories = parse_calorie_groups(&lines);

    let max = problem1(&calories);
    println!("single highest: {max}");

    let three = problem2(&calories);
    println!("three highest: {three}");
}

fn parse_calorie_groups(s: &str) -> Vec<u32> {
    fn number(s: &str) -> IResult<&str, u32> {
        map_res(digit1, |x| u32::from_str_radix(x, 10))(s)
    }

    let mut parser = separated_list1(
        line_ending,
        fold_many0(
            terminated(number, alt((line_ending, eof))),
            || 0,
            |x, y| x + y,
        ),
    );

    let (_, mut v) = parser(s).unwrap();
    v.sort_by_key(|x| Reverse(*x));
    v
}

fn problem1(cal: &[u32]) -> u32 {
    *cal.first().unwrap()
}

fn problem2(cal: &[u32]) -> u32 {
    cal.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::parse_calorie_groups;
    #[test]
    fn first() {
        let lines = get_raw_input();
        let calories = parse_calorie_groups(&lines);
        let max = crate::problem1(&calories);
        assert_eq!(max, 24000)
    }

    #[test]
    fn second() {
        let lines = get_raw_input();
        let calories = parse_calorie_groups(&lines);
        let max = crate::problem2(&calories);
        assert_eq!(max, 45000)
    }
}
