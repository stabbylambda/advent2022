use std::fmt::Debug;

pub struct Map<T> {
    pub points: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
pub type Coord = (usize, usize);

pub trait CoordExt {
    fn to_position(&self, width: usize) -> usize;
}
impl CoordExt for Coord {
    fn to_position(&self, width: usize) -> usize {
        let (x, y) = self;
        y * width + x
    }
}

impl<T> Map<T> {
    pub fn new(points: Vec<Vec<T>>) -> Map<T> {
        let height = points.len();
        let width = points[0].len();
        Map {
            points,
            height,
            width,
        }
    }

    pub fn get(&self, (x, y): Coord) -> &T {
        &self.points[y][x]
    }

    pub fn neighbors(&self, (x, y): Coord) -> Vec<Coord> {
        let mut v = Vec::new();
        if y != 0 {
            v.push((x, y - 1));
        }

        if x != 0 {
            v.push((x - 1, y));
        }

        if y != self.height - 1 {
            v.push((x, y + 1));
        }

        if x != self.width - 1 {
            v.push((x + 1, y));
        }
        v
    }
}

impl<T> Debug for Map<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.points[..] {
            for col in &row[..] {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
