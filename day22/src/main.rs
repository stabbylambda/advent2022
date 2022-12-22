use common::get_raw_input;
use ndarray::prelude::*;
use parsing::parse_grid;

use crate::parsing::parse;

pub mod parsing;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = (Array2<Space>, Vec<Instruction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Void,
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Walk(u32),
    TurnLeft,
    TurnRight,
}

fn print_map(grid: &Array2<Space>, player_position: &Position) {
    for (y, row) in grid.outer_iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if [y, x] == player_position.coords {
                let player_char = match player_position.heading {
                    Heading::Right => ">",
                    Heading::Down => "v",
                    Heading::Left => "<",
                    Heading::Up => "^",
                };
                print!("{player_char}");
                continue;
            }
            match space {
                Space::Void => print!(" "),
                Space::Empty => print!("."),
                Space::Wall => print!("#"),
            }
        }
        println!();
    }
    println!("==================");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Heading {
    Right,
    Down,
    Left,
    Up,
}

impl TryFrom<u32> for Heading {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value % 4 {
            0 => Heading::Right,
            1 => Heading::Down,
            2 => Heading::Left,
            3 => Heading::Up,
            _ => panic!("Not a valid num to heading conversion"),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    coords: [usize; 2],
    heading: Heading,
}

impl Position {
    fn get_start(grid: &Array2<Space>) -> Position {
        // find the starting position as the first Empty square on the first row of the board
        Position {
            coords: [
                0,
                grid.row(0).iter().position(|&x| x == Space::Empty).unwrap(),
            ],
            heading: Heading::Right,
        }
    }

    fn rotate(&mut self, instruction: &Instruction) -> Heading {
        let x = match instruction {
            Instruction::TurnLeft => 3,
            Instruction::TurnRight => 1,
            _ => 0,
        };
        self.heading = (self.heading as u32 + x).try_into().unwrap();
        self.heading
    }

    fn get_password(&self) -> u32 {
        // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
        let first = 1000 * (self.coords[0] + 1) as u32;
        let second = 4 * (self.coords[1] + 1) as u32;
        let third = self.heading as u32;

        first + second + third
    }

    fn walk(&mut self, steps: u32, grid: &Array2<Space>) {
        // get the correct axis to look at and slice the array on that axis
        let (axis, idx, rev) = match self.heading {
            Heading::Up => (Axis(1), 0, true),
            Heading::Down => (Axis(1), 0, false),
            Heading::Left => (Axis(0), 1, true),
            Heading::Right => (Axis(0), 1, false),
        };

        let slice = grid.index_axis(axis, self.coords[axis.0]);
        dbg!(&slice);

        let mut s = 0u32;
        while s < steps {
            // look at the next space
            let step = if rev { slice.len() - 1 } else { 1 };
            let next = (self.coords[idx] + step) % slice.len();

            match slice.get(next) {
                Some(Space::Wall) => break,
                Some(Space::Empty) => {
                    // println!(
                    //     "We can move 1 {:?} to ({}, {})",
                    //     player_position.heading,
                    //     player_position.coords[0],
                    //     player_position.coords[1]
                    // );
                    s += 1;
                    self.coords[idx] = next;
                }
                Some(Space::Void) => {
                    let next_non_void = if rev {
                        slice
                            .iter()
                            .rev()
                            .cycle()
                            .find(|&&x| x != Space::Void)
                            .unwrap()
                    } else {
                        slice.iter().cycle().find(|&&x| x != Space::Void).unwrap()
                    };

                    if *next_non_void == Space::Wall {
                        break;
                    }

                    // is the next non-void space a wall?
                    self.coords[idx] = next;
                }
                None => todo!(),
            }
        }
    }
}

#[test]
#[ignore = "reason"]
fn walk_up() {
    let map = ". 
# 
..
.#
..";

    let grid = parse_grid(map).unwrap().1;

    let mut p = Position {
        coords: [0, 0],
        heading: Heading::Up,
    };

    // can walk over the top of the map
    p.walk(5, &grid);
    assert_eq!(p.coords, [2, 0]);

    // now move over one
    p.coords = [2, 1];

    // can walk over the top, over a void, and hit a wall
    p.walk(5, &grid);
    assert_eq!(p.coords, [4, 1]);
}

#[test]
fn walk_left() {
    let map = "  .#.
  ..#";

    let grid = parse_grid(map).unwrap().1;

    let mut p = Position {
        coords: [0, 2],
        heading: Heading::Left,
    };

    // // can walk over void left
    // p.walk(3, &grid);
    // assert_eq!(p.coords, [0, 4]);

    p.coords = [1, 3];

    p.walk(10, &grid);
    print_map(&grid, &p);
    assert_eq!(p.coords, [1, 2]);
}

#[test]
#[ignore = "reason"]
fn rotate() {
    let mut p = Position {
        coords: [0, 0],
        heading: Heading::Up,
    };

    let x: Vec<Heading> = (0..4).map(|x| p.rotate(&Instruction::TurnLeft)).collect();
    assert_eq!(
        x,
        vec![Heading::Left, Heading::Down, Heading::Right, Heading::Up]
    );
    let x: Vec<Heading> = (0..4).map(|x| p.rotate(&Instruction::TurnRight)).collect();
    assert_eq!(
        x,
        vec![Heading::Right, Heading::Down, Heading::Left, Heading::Up]
    );
}

fn problem1(input: &Input) -> u32 {
    let (grid, moves) = input;
    let mut player_position = Position::get_start(grid);

    for x in moves.iter() {
        println!("{x:?}");
        match x {
            Instruction::TurnLeft | Instruction::TurnRight => {
                player_position.rotate(x);
            }
            Instruction::Walk(steps) => player_position.walk(*steps, grid),
        }
        // print_map(&grid, &player_position);
    }

    player_position.get_password()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    #[ignore = "reason"]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 6032)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
