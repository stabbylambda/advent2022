use std::cmp::Reverse;

use common::get_input_strings;
use nom::{
    branch::alt, character::complete::char, combinator::map, sequence::separated_pair, IResult,
};
fn main() {
    let lines = get_input_strings();
    let hands: Vec<(Hand, Hand)> = lines.iter().map(|x| get_pair(x).unwrap().1).collect();
    let score: u32 = hands.iter().map(|(o, s)| s.score(*o)).sum();

    println!("score: {score}");

    // let three = get_max_three(&lines);
    // println!("three highest: {three}");
}

fn get_hand(s: &str) -> IResult<&str, Hand> {
    let rock = map(alt((char('A'), char('X'))), |_| Hand::Rock);
    let paper = map(alt((char('B'), char('Y'))), |_| Hand::Paper);
    let scissors = map(alt((char('C'), char('Z'))), |_| Hand::Scissors);
    alt((rock, paper, scissors))(s)
}

fn get_pair(s: &str) -> IResult<&str, (Hand, Hand)> {
    separated_pair(get_hand, char(' '), get_hand)(s)
}

#[derive(Clone, Copy, Debug)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn score(&self, other: Hand) -> u32 {
        let score = match (self, other) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 0,
            (Hand::Rock, Hand::Scissors) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scissors) => 0,
            (Hand::Scissors, Hand::Rock) => 0,
            (Hand::Scissors, Hand::Paper) => 6,
            (Hand::Scissors, Hand::Scissors) => 3,
        };
        score + (*self as u32)
    }
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{get_pair, Hand};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let hands: Vec<(Hand, Hand)> = lines.iter().map(|x| get_pair(x).unwrap().1).collect();
        let score: u32 = hands.iter().map(|(o, s)| s.score(*o)).sum();
        assert_eq!(score, 15)
    }

    #[test]
    fn second() {
        Hand::Rock.score(Hand::Rock);
        Hand::Rock.score(Hand::Paper);
        Hand::Rock.score(Hand::Scissors);

        Hand::Paper.score(Hand::Rock);
        Hand::Paper.score(Hand::Paper);
        Hand::Paper.score(Hand::Scissors);

        Hand::Scissors.score(Hand::Rock);
        Hand::Scissors.score(Hand::Paper);
        Hand::Scissors.score(Hand::Scissors);
    }
}
