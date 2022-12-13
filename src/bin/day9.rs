use std::{
    collections::HashSet,
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_next_to(&mut self, other: &Point) {
        if (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 {
            return;
        }

        if self.x == other.x {
            if self.y > other.y + 1 {
                self.y = other.y + 1;
            } else if self.y < other.y - 1 {
                self.y = other.y - 1;
            }
        } else if self.y == other.y {
            if self.x > other.x + 1 {
                self.x = other.x + 1;
            } else if self.x < other.x - 1 {
                self.x = other.x - 1;
            }
        } else {
            if self.x > other.x {
                if self.y > other.y {
                    self.x -= 1;
                    self.y -= 1;
                } else {
                    self.x -= 1;
                    self.y += 1;
                }
            } else {
                if self.y > other.y {
                    self.x += 1;
                    self.y -= 1;
                } else {
                    self.x += 1;
                    self.y += 1;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut rope: Vec<Point> = iter::repeat(Point { x: 0, y: 0 }).take(10).collect();
    let mut tail_positions: HashSet<Point> = HashSet::new();
    tail_positions.insert(*rope.last().unwrap());

    let file = fs::File::open(path::Path::new("./data/day9.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let mut items = line.split_ascii_whitespace();
        let direction = items.next().unwrap();
        let steps: usize = items.next().unwrap().parse()?;

        let offset: (i32, i32) = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unknown direction {}", direction),
        };

        for _ in 0..steps {
            let mut new_rope = rope.clone();
            new_rope[0] = Point {
                x: rope[0].x + offset.0,
                y: rope[0].y + offset.1,
            };
            for idx in 1..rope.len() {
                let upstream = new_rope[idx - 1];
                new_rope[idx].move_next_to(&upstream);
            }

            tail_positions.insert(*new_rope.last().unwrap());
            rope = new_rope;
        }
    }

    dbg!(rope);
    dbg!(&tail_positions.len());

    Ok(())
}
