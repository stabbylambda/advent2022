use common::get_raw_input;
use ndarray::{prelude::*, OwnedRepr};
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom_u32};
use nom::combinator::map;
use nom::sequence::{terminated, tuple};
use nom::{multi::separated_list0, IResult};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

struct Grid {
    grid: ArrayBase<OwnedRepr<u32>, Dim<[usize; 3]>>,
}
type Input = Grid;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<(usize, usize, usize)>> = separated_list0(
        newline,
        tuple((
            terminated(map(nom_u32, |x| x as usize), tag(",")),
            terminated(map(nom_u32, |x| x as usize), tag(",")),
            map(nom_u32, |x| x as usize),
        )),
    )(input);

    let result = result.unwrap().1;

    points_to_grid(&result[..])
}

const AIR: u32 = 0;
const LAVA: u32 = 1;
const VACCUUM: u32 = 2;

fn points_to_grid(points: &[(usize, usize, usize)]) -> Grid {
    let mut grid = Array3::<u32>::zeros((30, 30, 30));
    for &(x, y, z) in points {
        grid[[x + 1, y + 1, z + 1]] = LAVA;
    }

    Grid { grid }
}

fn problem1(input: &Input) -> usize {
    let mut total_empty = 0;
    for x in 1..30 {
        for y in 1..30 {
            for z in 1..30 {
                if input.grid[[x, y, z]] == LAVA {
                    // this is a voxel, check all its neighbors
                    let neighbors = [
                        [x + 1, y, z],
                        [x - 1, y, z],
                        [x, y + 1, z],
                        [x, y - 1, z],
                        [x, y, z + 1],
                        [x, y, z - 1],
                    ];

                    for neighbor in neighbors {
                        if input.grid[neighbor] == AIR {
                            total_empty += 1;
                        }
                    }
                }
            }
        }
    }

    total_empty
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, points_to_grid, problem1, problem2};
    #[test]
    fn supermini() {
        let v = vec![(1, 1, 1)];
        let g = points_to_grid(&v);
        let result = problem1(&g);
        assert_eq!(result, 6)
    }

    #[test]
    fn mini() {
        let v = vec![(1, 1, 1), (2, 1, 1)];
        let g = points_to_grid(&v);
        let result = problem1(&g);
        assert_eq!(result, 10)
    }
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 64)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 58)
    }
}
