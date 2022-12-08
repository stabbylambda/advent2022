use common::get_input_strings;
fn main() {
    let lines = get_input_strings();

    let score = problem1(&lines);
    println!("problem 1 score: {score}");

    let score = problem2(&lines);
    println!("problem 2 score: {score}");
}

struct TreeGrid {
    trees: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

type Tree = (usize, usize, u32);
#[derive(Debug)]
struct TreeNeighbors {
    tree: Tree,
    north: Vec<Tree>,
    south: Vec<Tree>,
    east: Vec<Tree>,
    west: Vec<Tree>,
}

impl TreeGrid {
    fn parse(lines: &[String]) -> TreeGrid {
        let trees: Vec<Vec<u32>> = lines
            .iter()
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let height = trees.len();
        let width = trees[0].len();

        TreeGrid {
            trees,
            height,
            width,
        }
    }

    fn orthogonal_trees(&self, x: usize, y: usize) -> TreeNeighbors {
        // check the vertical and horizontal from this tree
        let tree = (x, y, self.trees[y][x]);
        let north: Vec<Tree> = (0..y).rev().map(|dy| (x, dy, self.trees[dy][x])).collect();
        let south: Vec<Tree> = (y + 1..self.height)
            .map(|dy| (x, dy, self.trees[dy][x]))
            .collect();
        let west: Vec<Tree> = (0..x).rev().map(|dx| (dx, y, self.trees[y][dx])).collect();
        let east: Vec<Tree> = (x + 1..self.width)
            .map(|dx| (dx, y, self.trees[y][dx]))
            .collect();

        TreeNeighbors {
            tree,
            north,
            south,
            east,
            west,
        }
    }
}

fn problem1(lines: &[String]) -> u32 {
    let grid = TreeGrid::parse(lines);
    let mut visible = 0;

    for (y, row) in grid.trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let neighbors = grid.orthogonal_trees(x, y);

            // check the vertical and horizontal from this tree
            let visible_from_north = neighbors.north.iter().all(|(_x, _y, h)| h < tree);
            let visible_from_south = neighbors.south.iter().all(|(_x, _y, h)| h < tree);
            let visible_from_west = neighbors.west.iter().all(|(_x, _y, h)| h < tree);
            let visible_from_east = neighbors.east.iter().all(|(_x, _y, h)| h < tree);

            if visible_from_north || visible_from_south || visible_from_east || visible_from_west {
                visible += 1;
            }
        }
    }

    visible
}

fn view(height: u32, neighbors: Vec<Tree>) -> u32 {
    let mut view = 0;
    for (_x, _y, h) in neighbors {
        view += 1;
        if h >= height {
            break;
        }
    }

    view
}

fn problem2(lines: &[String]) -> u32 {
    let grid = TreeGrid::parse(lines);
    let scores: Vec<u32> = grid
        .trees
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tree)| {
                    let neighbors = grid.orthogonal_trees(x, y);
                    let (_x, _y, height) = neighbors.tree;

                    let north = view(height, neighbors.north);
                    let south = view(height, neighbors.south);
                    let east = view(height, neighbors.east);
                    let west = view(height, neighbors.west);

                    north * south * east * west
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    *scores.iter().max().unwrap()
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
        assert_eq!(result, 8)
    }
}
