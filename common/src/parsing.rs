use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn number(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |x| u32::from_str_radix(x, 10))(s)
}
