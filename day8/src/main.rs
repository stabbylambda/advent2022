use common::get_raw_input;
use nom::{
    character::complete::{anychar, newline},
    combinator::{map, map_opt},
    multi::{many1, separated_list1},
    IResult,
};
fn main() {
    let lines = get_raw_input();
    let input = TreeGrid::parse(&lines);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
struct TreeGrid {
    trees: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

type Tree<'a> = (usize, usize, &'a u32);
#[derive(Debug)]
struct TreeNeighbors<'a> {
    north: Vec<Tree<'a>>,
    south: Vec<Tree<'a>>,
    east: Vec<Tree<'a>>,
    west: Vec<Tree<'a>>,
}

impl TreeGrid {
    fn parse(lines: &str) -> TreeGrid {
        let parsed: IResult<&str, TreeGrid> = map(
            separated_list1(newline, many1(map_opt(anychar, |c| c.to_digit(10)))),
            |trees| {
                let height = trees.len();
                let width = trees[0].len();

                TreeGrid {
                    trees,
                    height,
                    width,
                }
            },
        )(lines);

        parsed.unwrap().1
    }

    fn orthogonal_trees<'a>(&'a self, tree: Tree<'a>) -> TreeNeighbors<'a> {
        let (x, y, _value) = tree;
        // check the vertical and horizontal from this tree
        let north: Vec<Tree> = (0..y).rev().map(|dy| (x, dy, &self.trees[dy][x])).collect();
        let south: Vec<Tree> = (y + 1..self.height)
            .map(|dy| (x, dy, &self.trees[dy][x]))
            .collect();
        let west: Vec<Tree> = (0..x).rev().map(|dx| (dx, y, &self.trees[y][dx])).collect();
        let east: Vec<Tree> = (x + 1..self.width)
            .map(|dx| (dx, y, &self.trees[y][dx]))
            .collect();

        TreeNeighbors {
            north,
            south,
            east,
            west,
        }
    }
}

fn problem1(grid: &TreeGrid) -> u32 {
    let mut visible = 0;

    for (y, row) in grid.trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let neighbors = grid.orthogonal_trees((x, y, tree));

            // check the vertical and horizontal from this tree
            let visible_from_north = neighbors.north.iter().all(|(_x, _y, h)| *h < tree);
            let visible_from_south = neighbors.south.iter().all(|(_x, _y, h)| *h < tree);
            let visible_from_west = neighbors.west.iter().all(|(_x, _y, h)| *h < tree);
            let visible_from_east = neighbors.east.iter().all(|(_x, _y, h)| *h < tree);

            if visible_from_north || visible_from_south || visible_from_east || visible_from_west {
                visible += 1;
            }
        }
    }

    visible
}

fn view((_x, _y, height): Tree, neighbors: Vec<Tree>) -> u32 {
    let mut view = 0;
    for (_x, _y, h) in neighbors {
        view += 1;
        if h >= height {
            break;
        }
    }

    view
}

fn problem2(grid: &TreeGrid) -> u32 {
    let scores: Vec<u32> = grid
        .trees
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, h)| {
                    let tree: Tree = (x, y, h);
                    let neighbors = grid.orthogonal_trees(tree);

                    let north = view(tree, neighbors.north);
                    let south = view(tree, neighbors.south);
                    let east = view(tree, neighbors.east);
                    let west = view(tree, neighbors.west);

                    north * south * east * west
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{problem1, problem2, TreeGrid};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = TreeGrid::parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 21)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = TreeGrid::parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 8)
    }
}
