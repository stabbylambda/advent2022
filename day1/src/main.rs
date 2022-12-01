use std::cmp::Reverse;

use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let max = get_max(&lines);
    println!("single highest: {max}");

    let three = get_max_three(&lines);
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

fn get_max(lines: &[String]) -> u32 {
    let cal = sort_calories(lines);
    *cal.first().unwrap()
}

fn get_max_three(lines: &[String]) -> u32 {
    let cal = sort_calories(lines);
    cal.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;
    #[test]
    fn first() {
        let lines = get_input_strings();
        let max = crate::get_max(&lines);
        assert_eq!(max, 24000)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let max = crate::get_max_three(&lines);
        assert_eq!(max, 45000)
    }
}
