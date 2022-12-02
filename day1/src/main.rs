use std::cmp::Reverse;

use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let max = problem1(&lines);
    println!("single highest: {max}");

    let three = problem2(&lines);
    println!("three highest: {three}");
}

fn sort_calories(lines: &[String]) -> Vec<u32> {
    let mut v = lines
        .split(|s| s == "") // split by blank lines
        .map(|group| {
            // total up all the strings in each grouping
            group.iter().map(|s| s.parse::<u32>().unwrap_or(0)).sum()
        })
        .collect::<Vec<u32>>();

    v.sort_by_key(|x| Reverse(*x));
    v
}

fn problem1(lines: &[String]) -> u32 {
    let cal = sort_calories(lines);
    *cal.first().unwrap()
}

fn problem2(lines: &[String]) -> u32 {
    let cal = sort_calories(lines);
    cal.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;
    #[test]
    fn first() {
        let lines = get_input_strings();
        let max = crate::problem1(&lines);
        assert_eq!(max, 24000)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let max = crate::problem2(&lines);
        assert_eq!(max, 45000)
    }
}
