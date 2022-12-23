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
}

impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            ' ' => Spot::Blank,
            '.' => Spot::Floor,
            '#' => Spot::Wall,
            _ => panic!("Unknown spot character"),
        }
    }
}

impl Debug for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, " "),
            Self::Floor => write!(f, "."),
            Self::Wall => write!(f, "#"),
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
            .get(0)
            .unwrap()
            .iter()
            .find_position(|&&spot| spot == Spot::Floor)
            .unwrap();
        State {
            x,
            y: 0,
            facing: Direction::Right,
        }
    }

    fn apply_instruction(self, i: Instruction, map: &Vec<Vec<Spot>>) -> Self {
        match i {
            Instruction::Forward(n) => match self.facing {
                Direction::Up => self.move_steps(n, 0, -1, map),
                Direction::Down => self.move_steps(n, 0, 1, map),
                Direction::Left => self.move_steps(n, -1, 0, map),
                Direction::Right => self.move_steps(n, 1, 0, map),
            },
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

    fn move_steps(
        self,
        steps: usize,
        x_offset: isize,
        y_offset: isize,
        map: &Vec<Vec<Spot>>,
    ) -> Self {
        let width: isize = map.get(0).unwrap().len() as isize;
        let height: isize = map.len() as isize;

        let mut result = self;
        for _ in 0..steps {
            let mut next_x = result.x as isize;
            let mut next_y = result.y as isize;
            loop {
                next_x += x_offset;
                if next_x < 0 {
                    next_x = width - 1;
                } else if next_x >= width {
                    next_x = 0;
                }

                next_y += y_offset;
                if next_y < 0 {
                    next_y = height - 1;
                } else if next_y >= height {
                    next_y = 0;
                }

                let tgt_spot = *map
                    .get(next_y as usize)
                    .unwrap()
                    .get(next_x as usize)
                    .unwrap();

                match tgt_spot {
                    Spot::Blank => continue,
                    Spot::Floor => {
                        result.x = next_x as usize;
                        result.y = next_y as usize;
                        break;
                    }
                    Spot::Wall => return result,
                }
            }
        }
        result
    }

    fn to_score(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + self.facing.to_score()
    }
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
                Some(_) => panic!("Unknown instruction alphanumeric character"),
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
    for i in instructions {
        state = state.apply_instruction(i, &map);
    }

    dbg!(&state);
    dbg!(&state.to_score());

    Ok(())
}
