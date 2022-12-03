use std::collections::HashSet;

use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
struct Rucksack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

impl Rucksack {
    fn get_shared(&self) -> Vec<char> {
        let s1: HashSet<char> = HashSet::from_iter(self.compartment1.iter().map(|c| *c));
        let s2: HashSet<char> = HashSet::from_iter(self.compartment2.iter().map(|c| *c));
        let common: Vec<char> = s1.intersection(&s2).map(|c| *c).collect();

        common
    }
}

fn get_rucksack(s: &str) -> Rucksack {
    let (compartment1, compartment2) = s.split_at(s.len() / 2);
    Rucksack {
        compartment1: compartment1.chars().map(|c| c as char).collect(),
        compartment2: compartment2.chars().map(|c| c as char).collect(),
    }
}

fn prioritize(c: &char) -> u32 {
    (*c as u32) - if c.is_lowercase() { 96 } else { 38 }
}

fn problem1(lines: &[String]) -> u32 {
    let rucksacks: Vec<Rucksack> = lines.iter().map(|s| get_rucksack(&s)).collect();
    let priorities = rucksacks
        .iter()
        .flat_map(|r| {
            r.get_shared()
                .iter()
                .map(|c| prioritize(c))
                .collect::<Vec<u32>>()
        })
        .sum();

    priorities
}

fn problem2(lines: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    #[test]
    fn test_get_rucksack() {
        let r = get_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(
            r.compartment1,
            "vJrwpWtwJgWr".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            r.compartment2,
            "hcsFMMfFFhFp".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn common_test() {
        let r = get_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(r.get_shared()[0], 'p');
    }

    use crate::{get_rucksack, problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let result = problem1(&lines);
        assert_eq!(result, 157);
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let result = problem2(&lines);
        assert_eq!(result, 0)
    }
}
