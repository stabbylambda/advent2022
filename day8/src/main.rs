use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

fn problem1(lines: &[String]) -> u32 {
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut visible = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // all trees on the edge are always visible obviously
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                visible += 1;
                continue;
            }

            // check the vertical and horizontal from this tree
            let visible_from_north = (0..y).map(|dy| grid[dy][x]).all(|h| h < *tree);
            let visible_from_south = (y + 1..height).map(|dy| grid[dy][x]).all(|h| h < *tree);
            let visible_from_west = (0..x).map(|dx| grid[y][dx]).all(|h| h < *tree);
            let visible_from_east = (x + 1..width).map(|dx| grid[y][dx]).all(|h| h < *tree);

            if visible_from_north || visible_from_south || visible_from_east || visible_from_west {
                visible += 1;
            }
        }
    }

    visible
}

fn problem2(lines: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use common::test::get_input_strings;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let lines = get_input_strings();
        let result = problem1(&lines);
        assert_eq!(result, 21)
    }

    #[test]
    fn second() {
        let lines = get_input_strings();
        let result = problem2(&lines);
        assert_eq!(result, 0)
    }
}
