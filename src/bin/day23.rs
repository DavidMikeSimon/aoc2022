use std::{
    collections::{HashSet, HashMap},
    convert::TryInto,
    error,
    fmt::{Debug, Write},
    fs,
    io::{self, BufRead},
    iter, path,
};

use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Offset {
    x: i32,
    y: i32,
}

const NORTH: Offset = Offset { x: 0, y: -1 };
const NORTHEAST: Offset = Offset { x: 1, y: -1 };
const EAST: Offset = Offset { x: 1, y: 0 };
const SOUTHEAST: Offset = Offset { x: 1, y: 1 };
const SOUTH: Offset = Offset { x: 0, y: 1 };
const SOUTHWEST: Offset = Offset { x: -1, y: 1 };
const WEST: Offset = Offset { x: -1, y: 0 };
const NORTHWEST: Offset = Offset { x: -1, y: -1 };

lazy_static! {
    static ref ALL_DIRECTIONS: Vec<Offset> = {
        vec![
            NORTH,
            NORTHEAST,
            EAST,
            SOUTHEAST,
            SOUTH,
            SOUTHWEST,
            WEST,
            NORTHWEST,
        ]
    };

    static ref MOVEMENTS_TO_CONSIDER: Vec<(Offset, Vec<Offset>)> = {
        vec![
            (NORTH, vec![NORTHEAST, NORTH, NORTHWEST]),
            (SOUTH, vec![SOUTHEAST, SOUTH, SOUTHWEST]),
            (WEST, vec![NORTHWEST, WEST, SOUTHWEST]),
            (EAST, vec![NORTHEAST, EAST, SOUTHEAST]),
        ]
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply_offset(&self, offset: Offset) -> Self {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }

    fn consider_move(&self, elves: &HashSet<Point>, round_offset: usize) -> Option<Self> {
        if ALL_DIRECTIONS.iter().find(|&&look_dir| elves.contains(&self.apply_offset(look_dir))).is_none() {
            return None;
        }

        for i in 0..MOVEMENTS_TO_CONSIDER.len() {
            let idx = (i + round_offset) % MOVEMENTS_TO_CONSIDER.len();
            let (move_dir, looking) = MOVEMENTS_TO_CONSIDER.get(idx).unwrap();
            if looking.iter().find(|&&look_dir| elves.contains(&self.apply_offset(look_dir))).is_none() {
                return Some(self.apply_offset(*move_dir))
            }
        }
        None
    }
}

fn get_aabb(elves: &HashSet<Point>) -> (i32, i32, i32, i32) {
    let start_elf = elves.iter().next().unwrap();
    let mut min_x = start_elf.x;
    let mut max_x = start_elf.x;
    let mut min_y = start_elf.y;
    let mut max_y = start_elf.y;

    for elf in elves {
        min_x = min_x.min(elf.x);
        max_x = max_x.max(elf.x);
        min_y = min_y.min(elf.y);
        max_y = max_y.max(elf.y);
    }

    (min_x, max_x, min_y, max_y)
}

fn get_dimensions(elves: &HashSet<Point>) -> (usize, usize) {
    let (min_x, max_x, min_y, max_y) = get_aabb(elves);
    ((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1)
}

fn print_map(elves: &HashSet<Point>) {
    let (min_x, max_x, min_y, max_y) = get_aabb(elves);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&Point{x, y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day23.txt"))?;
    let mut elves: HashSet<Point> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| match c {
                '#' => Some(Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() }),
                _ => None,
            }).collect::<HashSet<_>>()
        })
        .collect();

    // println!();
    // println!("INITIAL STATE");
    // print_map(&elves);
    
    let mut round = 0;
    loop {
        // key is proposed location, value is original elf position
        let mut proposals: HashMap<Point, Vec<Point>> = HashMap::new();
        for elf in &elves {
            let result = elf.consider_move(&elves, round);
            if let Some(target) = result {
                proposals.entry(target).or_default().push(*elf);
            }
        }

        round += 1;

        if proposals.len() == 0 {
            break;
        }

        for (target, source_elves) in proposals {
            if source_elves.len() == 1 {
                elves.remove(source_elves.get(0).unwrap());
                elves.insert(target);
            }
        }

        // println!();
        // println!("END OF ROUND {}", round+1);
        // print_map(&elves);
    }
    
    dbg!(round);

    // let (width, height) = get_dimensions(&elves);
    // let area = width*height;
    // let score = area - elves.len();
    // dbg!(score);

    Ok(())
}
