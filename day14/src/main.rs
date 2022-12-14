use cavemap::{CaveMap, Path, Tile};
use common::{get_raw_input, nom::coord};
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    IResult,
};

pub mod cavemap;

fn main() {
    let input = get_raw_input();
    let mut input = parse(&input);

    let score = problem1(&mut input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = cavemap::CaveMap;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = map(
        separated_list1(newline, map(separated_list1(tag(" -> "), coord), Path::new)),
        CaveMap::new,
    )(input);

    result.unwrap().1
}

#[derive(PartialEq, Eq)]
enum SandResult {
    Abyss,
    Settled,
}

fn simulate_sand(input: &mut Input) -> SandResult {
    // every sand particle starts at the source
    let (mut x, mut y) = input.source;

    // check if we're off the left or right edge or below the lowest rock
    while 0 < x && x < input.map.width && y < input.map.height - 1 {
        // check the next square down
        let down = input.map.get((x, y + 1)).data;
        let diag_left = input.map.get((x - 1, y + 1)).data;
        let diag_right = input.map.get((x + 1, y + 1)).data;

        if down == &Tile::Air {
            y += 1;
        } else if diag_left == &Tile::Air {
            // move left
            x -= 1;
            y += 1;
        } else if diag_right == &Tile::Air {
            // move right
            x += 1;
            y += 1;
        } else {
            // we can't move down, diagonal left, or diagonal right so we settle here
            input.map.set((x, y), Tile::Sand);
            return SandResult::Settled;
        }
    }
    // we're off the map, so return Abyss
    SandResult::Abyss
}

fn problem1(input: &mut Input) -> u32 {
    let mut grains = 0;

    while let SandResult::Settled = simulate_sand(input) {
        grains += 1;
    }

    dbg!(&input.map);
    grains
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let mut input = parse(&input);
        let result = problem1(&mut input);
        assert_eq!(result, 24)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
