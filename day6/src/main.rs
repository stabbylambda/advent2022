use std::collections::BTreeSet;

use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

fn unique_string(count: usize, line: &str) -> u32 {
    let all_chars = line.chars().collect::<Vec<char>>();
    let control = all_chars.windows(count).enumerate().find(|(_, chars)| {
        let set: BTreeSet<&char> = chars.iter().collect();
        set.len() == count
    });

    let (idx, _sequence) = control.unwrap();
    (idx + count) as u32
}

fn problem1(lines: &[String]) -> u32 {
    unique_string(4, &lines[0])
}

fn problem2(lines: &[String]) -> u32 {
    unique_string(14, &lines[0])
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let result = problem1(&lines);
        assert_eq!(result, 7)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let result = problem2(&lines);
        assert_eq!(result, 19)
    }
}
