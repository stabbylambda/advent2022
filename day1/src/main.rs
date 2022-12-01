use common::{get_input, get_test_input};
fn main() {
    let lines = get_input(|s| s.to_owned());
    let max = get_max(&lines);

    println!("single highest: {max}");

    let three = get_max_three(&lines);

    println!("three highest: {three}");
}

fn sort_calories(lines: &[String]) -> Vec<i32> {
    let mut v: Vec<i32> = lines
        .split(|s| s == "") // split by blank lines
        .map(|group| {
            // total up all the strings in each grouping
            group
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect();

    v.sort();
    v.reverse();
    v
}

fn get_max(lines: &[String]) -> i32 {
    let cal = sort_calories(lines);
    *cal.first().unwrap()
}

fn get_max_three(lines: &[String]) -> i32 {
    let cal = sort_calories(lines);
    cal.iter().take(3).fold(0, |acc, x| acc + *x)
}

#[test]
fn first() {
    let lines = get_test_input(|s| s.to_owned());
    let max = get_max(&lines);
    assert_eq!(max, 24000)
}

#[test]
fn second() {
    let lines = get_test_input(|s| s.to_owned());
    let max = get_max_three(&lines);
    assert_eq!(max, 45000)
}
