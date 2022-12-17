#![feature(drain_filter)]

use itertools::{Itertools, MinMaxResult};
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{btree_map::Range, HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter,
    ops::RangeInclusive,
    path,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

// const MAX_SEARCH: isize = 20;
const MAX_SEARCH: isize = 4000000;

fn merge_range(row: &mut Vec<RangeInclusive<isize>>, range: &RangeInclusive<isize>) -> bool {
    let intersecting: Vec<_> = row
        .drain_filter(|r| {
            RangeInclusive::contains(r, range.start())
                || RangeInclusive::contains(range, r.start())
                || *r.start() == range.end() + 1
                || *range.start() == r.end() + 1
        })
        .collect();

    let min = intersecting
        .iter()
        .map(|r| r.start())
        .min()
        .unwrap_or(range.start())
        .min(range.start());
    let max = intersecting
        .iter()
        .map(|r| r.end())
        .max()
        .unwrap_or(range.end())
        .max(range.end());

    if *min <= 0 && *max >= MAX_SEARCH {
        return false;
    }

    row.push(*min..=*max);
    true
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(
        r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)",
    )
    .unwrap();

    let mut dead_rows: HashSet<isize> = HashSet::new();
    let mut rows: HashMap<isize, Vec<RangeInclusive<isize>>> = HashMap::new();

    let file = fs::File::open(path::Path::new("./data/day15.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let cap = re.captures(&line).unwrap();
        let sensor = Point {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
        };
        let beacon = Point {
            x: cap.get(3).unwrap().as_str().parse().unwrap(),
            y: cap.get(4).unwrap().as_str().parse().unwrap(),
        };

        let sb_dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        let min_y = (sensor.y - sb_dist).max(0);
        let max_y = (sensor.y + sb_dist).min(MAX_SEARCH);
        for y in min_y..=max_y {
            if dead_rows.contains(&y) {
                continue;
            }
            let sy_dist = (sensor.y - y).abs();
            let half_width = sb_dist - sy_dist;
            let lower_x = (sensor.x - half_width).max(0);
            let upper_x = (sensor.x + half_width).min(MAX_SEARCH);
            if lower_x > 0 || upper_x < MAX_SEARCH {
                let inserted = merge_range(rows.entry(y).or_default(), &(lower_x..=upper_x));
                if !inserted {
                    rows.remove(&y);
                    dead_rows.insert(y);
                }
            } else {
                dead_rows.insert(y);
                rows.remove(&y);
            }
        }
    }

    dbg!(rows.iter().sorted_by_key(|(k, _)| *k));
    // dbg!(dead_rows.iter().sorted());

    Ok(())
}
