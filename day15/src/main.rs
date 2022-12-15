use std::collections::HashSet;

use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input, 2_000_000);
    println!("problem 1 score: {score}");

    let score = problem2(&input, 4_000_000);
    println!("problem 2 score: {score}");
}

type Input = Vec<(Point, Point)>;

fn parse_coord(input: &str) -> IResult<&str, Point> {
    separated_pair(
        preceded(tag("x="), nom_i32),
        tag(", "),
        preceded(tag("y="), nom_i32),
    )(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        preceded(
            tag("Sensor at "),
            separated_pair(parse_coord, tag(": closest beacon is at "), parse_coord),
        ),
    )(input);

    result.unwrap().1
}

type Point = (i32, i32);
trait PointExt {
    fn manhattan(&self, p: &Point) -> u32;
}
impl PointExt for Point {
    fn manhattan(&self, (x2, y2): &Point) -> u32 {
        let (x1, y1) = self;
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }
}

fn get_coverage_area(input: &Input, row: i32) -> HashSet<Point> {
    let beacons: HashSet<&Point> = input.iter().map(|(_s, b)| b).collect();
    let sensors: Vec<(&Point, u32)> = input
        .iter()
        .filter_map(|(s @ (_sx, sy), b)| {
            // only consider the beacons within manhattan distance of this row
            let beacon_dist = s.manhattan(b);
            let y_in_range = sy - (beacon_dist as i32) <= row && row <= sy + (beacon_dist as i32);

            if y_in_range {
                println!("Sensor @ {s:?}+-{beacon_dist} can see row {row}");
                Some((s, beacon_dist))
            } else {
                println!("Sensor @ {s:?}+-{beacon_dist} has no coverage of row {row}");
                None
            }
        })
        .collect();

    let mut coverage_points: HashSet<(i32, i32)> = HashSet::new();
    for (&s @ (sx, sy), dist) in sensors {
        let y_distance = row.abs_diff(sy);
        let remaining_x_distance = y_distance.abs_diff(dist);

        let start_x = sx - (remaining_x_distance as i32);
        let end_x = sx + (remaining_x_distance as i32);

        println!("Sensor {s:?} is {y_distance} away from row {row}. Generating points from {start_x}..{end_x}");

        for px in start_x..=end_x {
            let p = (px, row);
            if beacons.contains(&p) {
                println!("{p:?} is already a beacon");
            } else {
                coverage_points.insert(p);
            }
        }
    }

    coverage_points
}

fn problem1(input: &Input, row: i32) -> usize {
    get_coverage_area(input, row).len()
}

fn problem2(input: &Input, max_search_area: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input, 10);
        assert_eq!(result, 26)
    }

    #[test]
    #[ignore]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input, 20);
        assert_eq!(result, 56000011)
    }
}
