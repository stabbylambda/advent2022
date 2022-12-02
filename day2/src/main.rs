use common::get_input_strings;
use nom::{
    branch::alt, character::complete::char, combinator::map, sequence::separated_pair, IResult,
};
fn main() {
    let lines = get_input_strings();
    let hands: Vec<(Hand, Hand)> = lines.iter().map(|x| naive_get_pair(x).unwrap().1).collect();
    let score: u32 = hands.iter().map(|(o, s)| s.naive_score(*o)).sum();

    println!("score: {score}");

    // let three = get_max_three(&lines);
    // println!("three highest: {three}");
}

fn naive_get_hand(s: &str) -> IResult<&str, Hand> {
    let rock = map(alt((char('A'), char('X'))), |_| Hand::Rock);
    let paper = map(alt((char('B'), char('Y'))), |_| Hand::Paper);
    let scissors = map(alt((char('C'), char('Z'))), |_| Hand::Scissors);
    alt((rock, paper, scissors))(s)
}

fn naive_get_pair(s: &str) -> IResult<&str, (Hand, Hand)> {
    separated_pair(naive_get_hand, char(' '), naive_get_hand)(s)
}

#[derive(Clone, Copy, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn naive_score(&self, other: Hand) -> u32 {
        let s = *self as u32;
        let o = other as u32;

        let number_score = if o == (s + 1) % 3 {
            0
        } else if o == s {
            3
        } else {
            6
        };

        number_score + s + 1
    }
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{naive_get_pair, Hand};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let hands: Vec<(Hand, Hand)> = lines.iter().map(|x| naive_get_pair(x).unwrap().1).collect();
        let score: u32 = hands.iter().map(|(o, s)| s.naive_score(*o)).sum();
        assert_eq!(score, 15)
    }
}
