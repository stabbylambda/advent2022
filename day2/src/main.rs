use common::get_input_strings;
use nom::{
    branch::alt,
    character::complete::{char, space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
fn main() {
    let lines = get_input_strings();
    let score = problem1(&lines);
    println!("problem 1 score: {score}");
    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

fn problem1(lines: &[String]) -> u32 {
    // using nom is overkill for this, but I figured there's gonna be a lot more parsing later so might as well
    // get some practice in
    fn naive_get_hand(s: &str) -> IResult<&str, Hand> {
        let rock = map(alt((char('A'), char('X'))), |_| Hand::Rock);
        let paper = map(alt((char('B'), char('Y'))), |_| Hand::Paper);
        let scissors = map(alt((char('C'), char('Z'))), |_| Hand::Scissors);
        alt((rock, paper, scissors))(s)
    }

    fn naive_get_pair(s: &str) -> IResult<&str, (Hand, Hand)> {
        separated_pair(naive_get_hand, char(' '), naive_get_hand)(s)
    }

    let hands: Vec<(Hand, Hand)> = lines.iter().map(|x| naive_get_pair(x).unwrap().1).collect();
    let score: u32 = hands.iter().map(|(o, s)| s.score(*o)).sum();
    score
}

fn problem2(lines: &[String]) -> u32 {
    fn get_hand(s: &str) -> IResult<&str, (Hand, Strategy)> {
        let hand = alt((
            map(char('A'), |_| Hand::Rock),
            map(char('B'), |_| Hand::Paper),
            map(char('C'), |_| Hand::Scissors),
        ));
        let strategy = alt((
            map(char('X'), |_| Strategy::Lose),
            map(char('Y'), |_| Strategy::Draw),
            map(char('Z'), |_| Strategy::Win),
        ));

        separated_pair(hand, space1, strategy)(s)
    }

    let plays: Vec<(Hand, Strategy)> = lines.iter().map(|x| get_hand(x).unwrap().1).collect();
    plays
        .iter()
        .map(|(o, s)| {
            let my_hand = s.get_hand(*o);
            my_hand.score(*o)
        })
        .sum()
}

#[derive(Clone, Copy, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Strategy {
    Win = 1,
    Lose = -1,
    Draw = 0,
}

impl Strategy {
    fn get_hand(&self, other: Hand) -> Hand {
        // there's definitely a way to mod math this one too, but I am laaaazy
        match (other, self) {
            (Hand::Rock, Strategy::Win) => Hand::Paper,
            (Hand::Rock, Strategy::Lose) => Hand::Scissors,
            (Hand::Rock, Strategy::Draw) => Hand::Rock,
            (Hand::Paper, Strategy::Win) => Hand::Scissors,
            (Hand::Paper, Strategy::Lose) => Hand::Rock,
            (Hand::Paper, Strategy::Draw) => Hand::Paper,
            (Hand::Scissors, Strategy::Win) => Hand::Rock,
            (Hand::Scissors, Strategy::Lose) => Hand::Paper,
            (Hand::Scissors, Strategy::Draw) => Hand::Scissors,
        }
    }
}

impl Hand {
    fn score(&self, other: Hand) -> u32 {
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

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let score = problem1(&lines);
        assert_eq!(score, 15)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let score = problem2(&lines);
        assert_eq!(score, 12)
    }
}
