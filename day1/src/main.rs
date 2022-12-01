use common::{get_input, get_test_input};
fn main() {
    let lines = get_input(|s| s.to_owned());
    let max = get_max(&lines);

    println!("{max:?}");
}

fn get_max(lines: &[String]) -> i32 {
    lines
        .split(|s| s == "") // split by blank lines
        .map(|group| {
            // total up all the strings in each grouping
            group
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .max()
        .unwrap()
}

#[test]
fn first() {
    let lines = get_test_input(|s| s.to_owned());
    let max = get_max(&lines);
    assert_eq!(max, 24000)
}
