use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum CellKind {
    Rock,
    Sand,
}

use itertools::{Itertools, MinMaxResult};
use CellKind::*;

type Grid = HashMap<Point, CellKind>;

fn draw_grid(grid: &Grid) {
    let minmax_x = grid.iter().map(|(point, _)| point.x).minmax();

    let (min_x, max_x) = match minmax_x {
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::OneElement(n) => (n, n),
        _ => {
            println!("No rocks found!");
            return;
        }
    };

    let max_y = grid.iter().map(|(point, _)| point.y).max().unwrap_or(0);

    for y in 0..=(max_y + 1) {
        let row_str = (min_x..=max_x)
            .map(|x| match grid.get(&Point { x, y }) {
                Some(&Rock) => '#',
                Some(&Sand) => 'o',
                None => '.',
            })
            .join("");
        println!("{}", &row_str);
    }
}

fn insert_sand(grid: &mut Grid) -> bool {
    let max_y = grid
        .iter()
        .filter_map(|(point, c)| match c {
            Rock => Some(point.y),
            _ => None,
        })
        .max()
        .unwrap_or(0);

    let mut pos = Point { x: 500, y: 0 };

    loop {
        if grid.get(&pos) == Some(&Sand) {
            return false;
        }

        if pos.y == max_y + 1 {
            grid.insert(pos, Sand);
            return true;
        }

        let mut found = false;
        for next_pos in &[
            Point {
                x: pos.x,
                y: pos.y + 1,
            },
            Point {
                x: pos.x - 1,
                y: pos.y + 1,
            },
            Point {
                x: pos.x + 1,
                y: pos.y + 1,
            },
        ] {
            if grid.get(next_pos) == None {
                pos = *next_pos;
                found = true;
                break;
            }
        }

        if !found {
            grid.insert(pos, Sand);
            return true;
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut grid: Grid = HashMap::new();

    let file = fs::File::open(path::Path::new("./data/day14.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line = line.trim();
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                let nums: Vec<usize> = s
                    .split(",")
                    .map(|num_str| num_str.parse().unwrap())
                    .collect();
                Point {
                    x: nums[0],
                    y: nums[1],
                }
            })
            .collect();

        for (point_a, point_b) in points.iter().tuple_windows() {
            if point_a.y == point_b.y {
                let min_x = point_a.x.min(point_b.x);
                let max_x = point_a.x.max(point_b.x);
                for x in min_x..=max_x {
                    grid.entry(Point { x, y: point_a.y }).or_insert(Rock);
                }
            } else {
                let min_y = point_a.y.min(point_b.y);
                let max_y = point_a.y.max(point_b.y);
                for y in min_y..=max_y {
                    grid.entry(Point { x: point_a.x, y }).or_insert(Rock);
                }
            }
        }
    }

    let mut sand_inserted: usize = 0;
    loop {
        if sand_inserted % 100 == 0 {
            println!("{}", sand_inserted);
        }

        let inserted = insert_sand(&mut grid);
        if !inserted {
            break;
        }
        sand_inserted += 1;
    }

    dbg!(sand_inserted);

    Ok(())
}
