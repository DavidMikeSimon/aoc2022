use std::{
    collections::HashSet,
    error, fs,
    hash::Hash,
    io::{self, BufRead},
    path,
};

use itertools::{Itertools, MinMaxResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day18.txt"))?;
    let points: HashSet<Point> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let nums: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();
            Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect();

    let (min_x, max_x) = match points.iter().map(|p| p.x).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::OneElement(n) => (n, n),
        _ => panic!("No x minmax"),
    };

    let (min_y, max_y) = match points.iter().map(|p| p.y).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::OneElement(n) => (n, n),
        _ => panic!("No y minmax"),
    };

    let (min_z, max_z) = match points.iter().map(|p| p.z).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::OneElement(n) => (n, n),
        _ => panic!("No z minmax"),
    };

    let offsets: Vec<(isize, isize, isize)> = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    let mut external_points: HashSet<Point> = HashSet::new();
    let mut to_add: HashSet<Point> = HashSet::new();
    let mut to_add_next: HashSet<Point> = HashSet::new();

    to_add.insert(Point {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    });

    while !to_add.is_empty() {
        for point in to_add.iter() {
            for offset in &offsets {
                let p2 = Point {
                    x: point.x + offset.0,
                    y: point.y + offset.1,
                    z: point.z + offset.2,
                };

                if points.contains(&p2)
                    || external_points.contains(&p2)
                    || to_add.contains(&p2)
                    || p2.x < min_x - 1
                    || p2.x > max_x + 1
                    || p2.y < min_y - 1
                    || p2.y > max_y + 1
                    || p2.z < min_z - 1
                    || p2.z > max_z + 1
                {
                    continue;
                }

                to_add_next.insert(p2);
            }
        }

        external_points.extend(to_add.drain());
        to_add = to_add_next;
        to_add_next = HashSet::new();
    }

    let surface_area: usize = points
        .iter()
        .map(|p| {
            offsets
                .iter()
                .map(|offset| {
                    let p2 = Point {
                        x: p.x + offset.0,
                        y: p.y + offset.1,
                        z: p.z + offset.2,
                    };

                    match external_points.contains(&p2) {
                        true => 1,
                        false => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum();

    dbg!(surface_area);

    Ok(())
}
