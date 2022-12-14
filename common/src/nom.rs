use nom::{
    bytes::complete::tag,
    character::complete::{anychar, u32 as nom_u32},
    combinator::{map, map_opt},
    sequence::separated_pair,
    IResult,
};

use crate::map::Coord;

pub fn single_digit(s: &str) -> IResult<&str, u32> {
    map_opt(anychar, |c| c.to_digit(10))(s)
}

pub fn coord(s: &str) -> IResult<&str, Coord> {
    map(separated_pair(nom_u32, tag(","), nom_u32), |(x, y)| {
        (x as usize, y as usize)
    })(s)
}
