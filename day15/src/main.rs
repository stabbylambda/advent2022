use std::collections::HashSet;

use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input, 2_000_000);
    println!("problem 1 score: {score}");

    let score = problem2(&input, 4_000_000);
    println!("problem 2 score: {score}");
}

#[derive(Debug)]
struct Input {
    sensors: Vec<Sensor>,
    beacons: HashSet<Point>,
}

fn parse_coord(input: &str) -> IResult<&str, Point> {
    separated_pair(
        preceded(tag("x="), nom_i64),
        tag(", "),
        preceded(tag("y="), nom_i64),
    )(input)
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Vec<(Point, Point)>> = separated_list1(
        newline,
        preceded(
            tag("Sensor at "),
            separated_pair(parse_coord, tag(": closest beacon is at "), parse_coord),
        ),
    )(input);

    let pairs = result.unwrap().1;
    let sensors = pairs
        .iter()
        .map(|&(s @ (sx, sy), b)| Sensor {
            x: sx,
            y: sy,
            dist: s.manhattan(&b),
        })
        .collect();
    let beacons = pairs.iter().map(|(_s, b)| b.clone()).collect();

    Input { sensors, beacons }
}

type Point = (i64, i64);
trait PointExt {
    fn manhattan(&self, p: &Point) -> i64;
}
impl PointExt for Point {
    fn manhattan(&self, (x2, y2): &Point) -> i64 {
        let (x1, y1) = self;
        (x1.abs_diff(*x2) + y1.abs_diff(*y2)) as i64
    }
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn get_coverage(&self, row: i64) -> Option<(i64, i64)> {
        let &Sensor { x, y, dist } = self;
        // only consider the rows within manhattan distance of this sensor
        let y_in_range = y - dist <= row && row <= y + dist;

        y_in_range.then(|| {
            // get the vertical distance between these two points
            let y_distance = row.abs_diff(y);
            // the horizontal difference is the rest of the manhattan distance
            let remaining_x_distance = y_distance.abs_diff(dist as u64);

            let start_x = x - (remaining_x_distance as i64);
            let end_x = x + (remaining_x_distance as i64);

            (start_x, end_x)
        })
    }
}

fn get_contiguous_ranges(ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // order by x
    ranges.sort();

    ranges.iter().fold(vec![], |mut acc, &r @ (ra, rb)| {
        // push the first item on the stack
        match acc.pop() {
            Some(previous @ (pa, pb)) => {
                if ra <= pb {
                    acc.push((pa, rb.max(pb)))
                } else {
                    acc.push(previous);
                    acc.push(r);
                }
            }
            None => acc.push(r),
        };

        acc
    })
}

fn problem1(input: &Input, row: i64) -> i64 {
    // count the beacons on the row
    let existing_beacons = input.beacons.iter().filter(|(_, by)| *by == row).count() as i64;

    // find the coverages on this particular row
    let mut coverages: Vec<(i64, i64)> = input
        .sensors
        .iter()
        .filter_map(|s| s.get_coverage(row))
        .collect();

    // smash the ranges together and get the sum of the distances between them
    let count = get_contiguous_ranges(&mut coverages)
        .iter()
        .map(|&(start, end)| start.abs_diff(end))
        .sum::<u64>() as i64;

    // I'm honestly not sure what I need the 1 for, but I need it...so...
    let total: i64 = count - existing_beacons + 1;
    total
}

/* There is for sure a much faster way of doing this. I'm probably doing too many allocations of
intermediate vectors and too many double iterations. But I got the right answer and I think I may not care anymore.
*/
fn problem2(input: &Input, max_search_area: i64) -> i64 {
    let (x, y) = (0..max_search_area)
        .into_par_iter()
        .find_map_any(|row| {
            // find the coverages on this particular row
            let mut coverages: Vec<(i64, i64)> = input
                .sensors
                .iter()
                .filter_map(|s| s.get_coverage(row))
                .collect();

            let ranges = get_contiguous_ranges(&mut coverages);
            (ranges.len() > 1).then(|| {
                // the hole in between these two ranges is the x value of the beacon
                let x = ranges[0].1 + 1;
                (x, row)
            })
        })
        .unwrap();

    let tuning_frequency = (x * 4_000_000) + y;
    tuning_frequency
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
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input, 20);
        assert_eq!(result, 56000011)
    }
}
