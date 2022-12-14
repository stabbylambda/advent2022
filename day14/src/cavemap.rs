use common::map::{Coord, Map};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Air,
    Sand,
    Rock,
    Source,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Sand => write!(f, "o"),
            Self::Rock => write!(f, "#"),
            Self::Source => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Coord>,
}

impl Path {
    pub fn new(segments: Vec<Coord>) -> Self {
        Self { segments }
    }
    pub fn get_min_x(&self) -> usize {
        self.segments.iter().map(|&(x, _)| x).min().unwrap()
    }

    pub fn get_max_x(&self) -> usize {
        self.segments.iter().map(|&(x, _)| x).max().unwrap()
    }

    pub fn get_min_y(&self) -> usize {
        0
    }

    pub fn get_max_y(&self) -> usize {
        self.segments.iter().map(|&(_, y)| y).max().unwrap()
    }

    pub fn all_points(&self) -> BTreeSet<Coord> {
        let mut points = BTreeSet::new();
        for pair in self.segments.windows(2) {
            let &[(x1, y1), (x2, y2)] = pair else {
                panic!()
            };

            let start_x = x1.min(x2);
            let end_x = x1.max(x2);

            let start_y = y1.min(y2);
            let end_y = y1.max(y2);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    points.insert((x, y));
                }
            }
        }

        points
    }
}

// normalize a point so that x is a value relative to the minimum x as the zero
fn translate((x, y): &Coord, min_x: usize) -> Coord {
    let new_x = x - min_x;
    (new_x, *y)
}
pub struct CaveMap {
    pub map: Map<Tile>,
    pub source: Coord,
}

impl CaveMap {
    pub fn new(paths: Vec<Path>) -> Self {
        let min_x = paths.iter().map(|x| x.get_min_x()).min().unwrap();
        let min_y = paths.iter().map(|x| x.get_min_y()).min().unwrap();
        let max_x = paths.iter().map(|x| x.get_max_x()).max().unwrap();
        let max_y = paths.iter().map(|x| x.get_max_y()).max().unwrap();

        let width = max_x - min_x;
        let height = max_y - min_y;

        // init the map with air
        let tiles = vec![vec![Tile::Air; width + 1]; height + 1];
        let mut map = Map::new(tiles);

        // place all the rocks
        for path in paths {
            for point in path.all_points() {
                let rock = translate(&point, min_x);
                map.set(rock, Tile::Rock);
            }
        }

        // place the source just for completeness
        let source = translate(&(500, 0), min_x);
        map.set(source, Tile::Source);

        Self { map, source }
    }
}
