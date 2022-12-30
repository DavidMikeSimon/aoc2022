use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    error,
    fmt::{Debug, Write},
    fs,
    io::{self, BufRead},
    iter,
    ops::Mul,
    path,
};

use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Offset {
    x: i32,
    y: i32,
}

impl Offset {
    fn is_vertical(&self) -> bool {
        self.x == 0
    }

    fn is_horizontal(&self) -> bool {
        self.y == 0
    }

    fn try_from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(NORTH),
            '>' => Some(EAST),
            'V' | 'v' => Some(SOUTH),
            '<' => Some(WEST),
            _ => None,
        }
    }
}

impl Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Self::Output {
        Offset {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

const NORTH: Offset = Offset { x: 0, y: -1 };
const EAST: Offset = Offset { x: 1, y: 0 };
const SOUTH: Offset = Offset { x: 0, y: 1 };
const WEST: Offset = Offset { x: -1, y: 0 };

lazy_static! {
    static ref ALL_DIRECTIONS: Vec<Offset> = { vec![NORTH, EAST, SOUTH, WEST,] };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply_offset(&self, offset: Offset) -> Self {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    fn wrap_x(&self, width: i32) -> Self {
        Point {
            x: self.x.rem_euclid(width),
            y: self.y,
        }
    }

    fn wrap_y(&self, height: i32) -> Self {
        Point {
            x: self.x,
            y: self.y.rem_euclid(height),
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut max_x: u8 = 0;
    let mut max_y: u8 = 0;
    let file = fs::File::open(path::Path::new("./data/day24.txt"))?;
    let raw_blizzards: HashSet<(Point, Offset)> = io::BufReader::new(file)
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .enumerate()
        .flat_map(|(y, line)| {
            if line.chars().nth(1) == Some('#') {
                return HashSet::new();
            }

            max_y = max_y.max(y.try_into().unwrap());

            line.chars()
                .filter(|&c| c != '#')
                .enumerate()
                .filter_map(|(x, c)| {
                    max_x = max_x.max(x.try_into().unwrap());

                    match Offset::try_from_char(c) {
                        Some(offset) => Some((
                            Point {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            offset,
                        )),
                        _ => None,
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect();
    
    let vert_blizzard_spots: HashSet<(u8, Point)> = raw_blizzards
        .iter()
        .filter(|(_, offset)| offset.is_vertical())
        .flat_map(|(blizzard_point, blizzard_offset)| {
            (0..=max_y as u8).map(move |y_phase| {
                (
                    y_phase,
                    blizzard_point
                        .apply_offset(*blizzard_offset * y_phase.try_into().unwrap())
                        .wrap_y((max_y + 1).try_into().unwrap()),
                )
            })
        })
        .collect();

    let horiz_blizzard_spots: HashSet<(u8, Point)> = raw_blizzards
        .iter()
        .filter(|(_, offset)| offset.is_horizontal())
        .flat_map(|(blizzard_point, blizzard_offset)| {
            (0..=max_x as u8).map(move |x_phase| {
                (
                    x_phase,
                    blizzard_point
                        .apply_offset(*blizzard_offset * x_phase.try_into().unwrap())
                        .wrap_x((max_x + 1).try_into().unwrap()),
                )
            })
        })
        .collect();
    
    let mut known_spots: HashSet<(u8, u8, Point)> = HashSet::new();
    let mut next_spots: HashSet<(u8, u8, Point)> = HashSet::new();
    let mut next_next_spots: HashSet<(u8, u8, Point)> = HashSet::new();
    next_next_spots.insert((0, 0, Point { x: 0, y: -1 }));

    let mut generations = 1;

    while !next_next_spots.is_empty() {
        println!("KNOWN SPOTS {}", known_spots.len());
        println!("GEN {}", generations);

        known_spots.extend(next_spots.drain());
        std::mem::swap(&mut next_next_spots, &mut next_spots);

        println!("NEXT SPOTS {}", next_spots.len());
        for &(x_phase, y_phase, point) in &next_spots {
            if point == (Point { x: max_x as i32, y: max_y as i32 }) {
                println!("Found exit in generation {}", generations);
                return Ok(());
            }

            let next_x_phase = (x_phase + 1) % (max_x + 1);
            let next_y_phase = (y_phase + 1) % (max_y + 1);

            for &dir in ALL_DIRECTIONS.iter().chain(iter::once(&Offset{x: 0, y: 0})) {
                let tgt = point.apply_offset(dir);

                if tgt.x < 0
                    || tgt.x > max_x.try_into().unwrap()
                    // Allow waiting in the start position, even though it's out of bounds
                    || (tgt.y < 0 && !(point == Point{x: 0, y: -1} && dir == Offset{x: 0, y: 0}))
                    || tgt.y > max_y.try_into().unwrap()
                    || vert_blizzard_spots.contains(&(next_y_phase, tgt))
                    || horiz_blizzard_spots.contains(&(next_x_phase, tgt))
                    || known_spots.contains(&(next_x_phase, next_y_phase, tgt))
                {
                    continue;
                }

                next_next_spots.insert((next_x_phase, next_y_phase, tgt));
            }
        }

        generations += 1;
    }

    Err("Unable to find exit".into())
}
