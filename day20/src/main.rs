use common::get_raw_input;
use nom::{
    character::complete::{i64 as nom_i64, newline},
    multi::separated_list0,
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

type Input = Vec<i64>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list0(newline, nom_i64)(input);

    result.unwrap().1
}

fn mix(numbers: &Input) -> Vec<i64> {
    // transform to pair of (idx, num)
    let mut numbers: Vec<(usize, &i64)> = numbers.iter().enumerate().collect();
    let size = numbers.len() as i64;

    for og_index_to_find in 0..numbers.len() {
        // find where the index originally was
        let current_index = numbers
            .iter()
            .position(|&(og_index, _x)| og_index == og_index_to_find)
            .unwrap();

        // mod math our way around the list to get to the new insertion index
        let (_og_index, num) = numbers[current_index];
        let new_index = (current_index as i64 + num).rem_euclid(size - 1) as usize;

        // remove and re-insert the value
        let number = numbers.remove(current_index);
        numbers.insert(new_index as usize, number);
    }

    // transform back to just numbers
    numbers.iter().map(|&(_i, &x)| x).collect()
}

fn problem1(input: &Input) -> i64 {
    let result = mix(input);
    let length = result.len();

    // find the 1, 2, and 3000th numbers from the current position of the 0 element
    let start = result.iter().position(|&x| x == 0).unwrap();
    (1..=3)
        .map(|x| result[((start + (x * 1000)) % length)])
        .sum()
}

fn problem2(_input: &Input) -> u32 {
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
        assert_eq!(result, 3)
    }

    #[test]
    #[ignore = "reason"]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
