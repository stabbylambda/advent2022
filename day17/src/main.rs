use std::{
    fmt::Display,
    ops::{BitAnd, BitOrAssign, Range},
};

use bitvec::{macros::internal::funty::Fundamental, prelude::*};
use common::get_raw_input;
use nom::{character::complete::anychar, combinator::map, multi::many1, IResult};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Jet>;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = many1(map(anychar, |x| match x {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => unreachable!(),
    }))(input);

    result.unwrap().1
}

#[derive(Debug, Clone, Copy)]
enum RockKind {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

impl Into<Rock> for RockKind {
    fn into(self) -> Rock {
        let rock = match self {
            // I could calculate these as hex values, but I like the visual representation
            RockKind::Horizontal => bitvec![u8, Lsb0;
                0,0,1,1,1,1,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::Plus => bitvec![u8, Lsb0;
                0,0,0,1,0,0,0,
                0,0,1,1,1,0,0,
                0,0,0,1,0,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::L => bitvec![u8, Lsb0;
                0,0,1,1,1,0,0,
                0,0,0,0,1,0,0,
                0,0,0,0,1,0,0,
                0,0,0,0,0,0,0,
            ],
            RockKind::Vertical => bitvec![u8, Lsb0;
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
                0,0,1,0,0,0,0,
            ],
            RockKind::Square => bitvec![u8, Lsb0;
                0,0,1,1,0,0,0,
                0,0,1,1,0,0,0,
                0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,
            ],
        };

        Rock { bits: rock }
    }
}

impl RockKind {
    fn all_kinds() -> Vec<RockKind> {
        vec![
            Self::Horizontal,
            Self::Plus,
            Self::L,
            Self::Vertical,
            Self::Square,
        ]
    }
}
struct Rock {
    bits: BitVec<u8>,
}

impl Rock {
    fn blow(&mut self, jet: &Jet, tower: &Tower, height: usize) {
        let left_wall = bitvec![u8, Lsb0;
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
                1,0,0,0,0,0,0,
        ];
        let right_wall = bitvec![u8, Lsb0;
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
                0,0,0,0,0,0,1,
        ];

        let on_left_wall = self.bits.clone().bitand(left_wall).any();
        let on_right_wall = self.bits.clone().bitand(right_wall).any();

        let mut new_bits = self.bits.clone();
        match jet {
            Jet::Left if !on_left_wall => &new_bits.shift_left(1),
            Jet::Right if !on_right_wall => &new_bits.shift_right(1),
            _ => {
                println!("  {height} Jet of gas pushes rock {jet:?}, but nothing happens (wall)");
                return;
            }
        };

        let rock_collision = tower.collision(&new_bits, height);
        if rock_collision {
            println!("  {height} Jet of gas pushes rock {jet:?}, but nothing happens (rock)");
            return;
        }

        println!("  {height} Jet of gas pushes rock {jet:?}");
        self.bits = new_bits;
    }
}
impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in (0..4).rev() {
            let range = (n * Tower::WIDTH)..(n * Tower::WIDTH + 7);
            let x = &self.bits[range];
            write!(f, "|")?;
            for y in x {
                let y = if y.as_bool() { "." } else { "#" };
                write!(f, "{y}")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "")
    }
}

struct Tower {
    bits: BitVec<u8>,
}

impl Tower {
    const WIDTH: usize = 7;

    fn new() -> Tower {
        // put a solid floor at the bottom of the tower
        let b = bitvec![u8, Lsb0;];
        Tower::from_bits(b)
    }

    fn from_bits(bits: BitVec<u8>) -> Tower {
        Tower { bits }
    }

    fn grow(&mut self) -> usize {
        let b = bitvec![u8, Lsb0; 0; 4*Tower::WIDTH];
        self.bits.extend(b);
        self.get_height() + 3
    }

    fn get_height(&self) -> usize {
        (self.bits.len() - self.bits.trailing_zeros() + Tower::WIDTH - 1) / Tower::WIDTH
    }

    fn get_row_range(&self, row: usize) -> Range<usize> {
        let start_index = row * Tower::WIDTH;
        let end_index = start_index + Tower::WIDTH;

        start_index..end_index
    }

    fn get_tower_slice(&self, row: usize) -> Range<usize> {
        let start_index = row * Tower::WIDTH;
        let end_index = self.bits.len().min(start_index + (4 * Tower::WIDTH));

        start_index..end_index
    }

    fn collision(&self, rock: &BitSlice<u8>, height: usize) -> bool {
        let r = self.get_tower_slice(height);
        let result = self.bits[r].to_bitvec().bitand(&rock[..]);
        let x = result.any();
        x
    }

    fn merge(&mut self, rock: &mut Rock, height: usize) {
        let r = self.get_tower_slice(height);

        self.bits[r].bitor_assign(&rock.bits);
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..=self.get_height()).rev() {
            let r = self.get_row_range(row);
            let x = &self.bits[r];
            write!(f, "{row:3} |")?;
            for y in x {
                let y = if y.as_bool() { "." } else { "#" };
                write!(f, "{y}")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "Tower height = {}", self.get_height())
    }
}

fn problem1(input: &Input) -> usize {
    let limit = 2022;
    let rocks = RockKind::all_kinds().into_iter().cycle().take(limit);
    let mut jets = input.iter().cycle();
    let mut tower = Tower::new();

    for kind in rocks {
        // println!("Tower height = {}, spawning {kind:?}", tower.get_height());
        let mut rock: Rock = kind.into();
        let mut height = tower.grow();

        loop {
            // blow the rock one way or another first
            let jet = jets.next().unwrap();
            rock.blow(jet, &tower, height);

            if height == 0 || tower.collision(&rock.bits, height - 1) {
                tower.merge(&mut rock, height);
                break;
            }

            // println!("  Rock falls 1 unit");
            height -= 1;
        }

        // println!("{tower}");
    }

    tower.get_height()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use bitvec::vec::BitVec;
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2, RockKind, Tower};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 3068)
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
