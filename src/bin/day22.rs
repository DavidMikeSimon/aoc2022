use std::{
    collections::HashMap,
    convert::TryInto,
    error,
    fmt::{Debug, Write},
    fs,
    io::{self, BufRead},
    iter, path,
};

use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Spot {
    Blank,
    Floor,
    Wall,
    Portal(char),
    PortalStart(char),
}

impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            ' ' => Spot::Blank,
            '.' => Spot::Floor,
            '#' => Spot::Wall,
            c => {
                if c.is_ascii_uppercase() {
                    Spot::Portal(c)
                } else if c.is_ascii_lowercase() {
                    Spot::PortalStart(c.to_ascii_uppercase())
                } else {
                    panic!("Unknown character {}", c)
                }
            }
        }
    }
}

impl Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, " "),
            Self::Floor => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Portal(c) => write!(f, "{}", c),
            Self::PortalStart(c) => write!(f, "{}", c.to_ascii_lowercase()),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Forward(usize),
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_score(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn turn_cw(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn turn_ccw(&self) -> Self {
        self.turn_cw().turn_cw().turn_cw()
    }

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'V',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    facing: Direction,
}

impl State {
    fn start(map: &Vec<Vec<Spot>>) -> Self {
        let (x, _) = map
            .get(1)
            .unwrap()
            .iter()
            .find_position(|&&spot| spot == Spot::Floor)
            .unwrap();
        State {
            x,
            y: 1,
            facing: Direction::Right,
        }
    }

    fn apply_instruction(self, i: Instruction, map: &Vec<Vec<Spot>>) -> Self {
        match i {
            Instruction::Forward(n) => self.move_steps(n, self.facing, map),
            Instruction::Clockwise => Self {
                facing: self.facing.turn_cw(),
                ..self
            },
            Instruction::Counterclockwise => Self {
                facing: self.facing.turn_ccw(),
                ..self
            },
        }
    }

    fn move_steps(self, steps: usize, facing: Direction, map: &Vec<Vec<Spot>>) -> Self {
        let mut result = self;

        for i in 0..steps {
            // println!("STEP {} OF {}", i+1, steps);
            let mut next_facing = result.facing;
            let mut next_x = result.x as isize;
            let mut next_y = result.y as isize;
            loop {
                let next_offset = next_facing.to_offset();
                next_x += next_offset.0;
                next_y += next_offset.1;

                let tgt_spot = map
                    .get(next_y as usize)
                    .map(|row| row.get(next_x as usize))
                    .unwrap_or(None);

                match tgt_spot {
                    None | Some(Spot::Blank) => {
                        panic!("Fell off the edge of the world")
                    }
                    Some(Spot::Floor) => {
                        result.x = next_x as usize;
                        result.y = next_y as usize;
                        result.facing = next_facing;
                        break;
                    }
                    Some(Spot::Wall) => return result,
                    Some(Spot::Portal(_) | Spot::PortalStart(_)) => {
                        (next_x, next_y, next_facing) =
                            portal_teleport((next_x, next_y), *tgt_spot.unwrap(), facing, map);
                    }
                }
            }
            // result.draw_map(map);
            // println!();
        }

        result
    }

    fn to_score(&self) -> usize {
        1000 * (self.y) + 4 * (self.x) + self.facing.to_score()
    }

    fn draw_map(&self, map: &Vec<Vec<Spot>>) {
        for (y, row) in map.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                if self.x == x && self.y == y {
                    print!("{}", self.facing.to_char());
                } else {
                    print!("{:?}", spot);
                }
            }
            println!();
        }
    }
}

fn portal_teleport(
    next_pos: (isize, isize),
    tgt_spot: Spot,
    facing: Direction,
    map: &Vec<Vec<Spot>>,
) -> (isize, isize, Direction) {
    let c = match tgt_spot {
        Spot::PortalStart(c) => c,
        Spot::Portal(c) => c,
        _ => panic!("Not a portal spot?"),
    };

    let mut portals: Vec<((isize, isize), (isize, isize))> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        let portal_spots: Vec<_> = row
            .iter()
            .enumerate()
            .filter(|(_, &s)| s == Spot::Portal(c) || s == Spot::PortalStart(c))
            .collect();
        if portal_spots.len() > 1 {
            let (x_first, spot_first) = portal_spots.first().unwrap();
            let (x_last, _) = portal_spots.last().unwrap();
            if **spot_first == Spot::PortalStart(c) {
                portals.push(((*x_first as isize, y as isize), (1, 0)));
            } else {
                portals.push(((*x_last as isize, y as isize), (-1, 0)));
            }
        }
    }
    for x in (0..map.first().unwrap().len()) {
        let portal_spots: Vec<_> = map
            .iter()
            .map(|row| row.get(x).unwrap())
            .enumerate()
            .filter(|(_, &s)| s == Spot::Portal(c) || s == Spot::PortalStart(c))
            .collect();
        if portal_spots.len() > 1 {
            let (y_first, spot_first) = portal_spots.first().unwrap();
            let (y_last, _) = portal_spots.last().unwrap();
            if **spot_first == Spot::PortalStart(c) {
                portals.push(((x as isize, *y_first as isize), (0, 1)));
            } else {
                portals.push(((x as isize, *y_last as isize), (0, -1)));
            }
        }
    }

    let portal_offset: usize = match tgt_spot {
        Spot::PortalStart(_) => 0,
        Spot::Portal(c) => match facing {
            Direction::Left | Direction::Right => {
                let r = map
                    .iter()
                    .map(|row| row.get(next_pos.0 as usize))
                    .enumerate()
                    .find(|(_, s)| s == &Some(&Spot::PortalStart(c)))
                    .unwrap();
                (next_pos.1 - (r.0 as isize)).abs() as usize
            }
            Direction::Up | Direction::Down => {
                let r = map
                    .get(next_pos.1 as usize)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .find(|(_, &s)| s == Spot::PortalStart(c))
                    .unwrap();
                (next_pos.0 - (r.0 as isize)).abs() as usize
            }
        },
        _ => panic!("Cannot teleport through this"),
    };

    let (in_portal_idx, _) = portals
        .iter()
        .find_position(|(portal_start_pos, portal_offset)| {
            (portal_start_pos.0 == next_pos.0 && portal_offset.0 == 0)
                || (portal_start_pos.1 == next_pos.1 && portal_offset.1 == 0)
        })
        .unwrap();

    let (_, out_portal) = portals
        .iter()
        .enumerate()
        .find(|(idx, _)| *idx != in_portal_idx)
        .unwrap();

    let out_portal_pos: (isize, isize) = (
        (out_portal.0).0 + (out_portal.1).0 * (portal_offset as isize),
        (out_portal.0).1 + (out_portal.1).1 * (portal_offset as isize),
    );

    let out_direction = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .find(|dir| {
        let offset = dir.to_offset();
        let out_dir_pos = (out_portal_pos.0 + offset.0, out_portal_pos.1 + offset.1);
        let spot = map
            .get(out_dir_pos.1 as usize)
            .map(|row| row.get(out_dir_pos.0 as usize))
            .unwrap_or(None);
        spot == Some(&Spot::Floor) || spot == Some(&Spot::Wall)
    })
    .unwrap();

    (out_portal_pos.0, out_portal_pos.1, *out_direction)
}

fn parse_map<'a>(lines: impl Iterator<Item = &'a String>) -> Vec<Vec<Spot>> {
    let mut max_width = 0;

    let mut map: Vec<Vec<Spot>> = lines
        .map(|line| {
            let row: Vec<Spot> = line.chars().map(|c| c.into()).collect();
            max_width = max_width.max(row.len());
            row
        })
        .collect();

    for row in map.iter_mut() {
        if row.len() < max_width {
            row.extend(iter::repeat(Spot::Blank).take(max_width - row.len()));
        }
    }

    map
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .batching(|it| {
            let maybe_num = it.peeking_take_while(|c| c.is_ascii_digit()).join("");
            if (maybe_num.len() > 0) {
                return Some(Instruction::Forward(maybe_num.parse().unwrap()));
            }

            match it.next() {
                Some('L') => Some(Instruction::Counterclockwise),
                Some('R') => Some(Instruction::Clockwise),
                Some(c) => panic!("Unknown instruction alphanumeric character '{}'", c),
                None => None,
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day22.txt"))?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.to_owned())
        .collect();

    let map = parse_map(lines.iter().take(lines.len() - 2));
    let instructions = parse_instructions(lines.iter().last().unwrap());

    let mut state = State::start(&map);
    state.draw_map(&map);
    println!();
    for i in instructions {
        // println!("----------- {:?}", i);
        state = state.apply_instruction(i, &map);
        // state.draw_map(&map);
        // println!();
    }

    dbg!(&state);
    dbg!(&state.to_score());

    Ok(())
}
