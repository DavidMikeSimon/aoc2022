#![feature(drain_filter)]
#![feature(let_chains)]

use core::time;
use itertools::{Itertools, MinMaxResult};
use regex::Regex;
use rust_dense_bitset::{BitSet, DenseBitSet};
use std::{
    cmp::{min, Ordering},
    collections::{btree_map::Range, HashMap, HashSet},
    convert::TryInto,
    error,
    fmt::Debug,
    fs,
    io::{self, BufRead},
    iter,
    ops::RangeInclusive,
    path,
};

enum Push {
    Left,
    Right,
}

impl Debug for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

fn get_grid_height(grid: &Vec<DenseBitSet>) -> usize {
    let mut h = grid.len();
    while h > 0 && grid.get(h - 1).unwrap().none() {
        h -= 1;
    }
    h
}

fn get_shape_width(shape: &Vec<DenseBitSet>) -> usize {
    shape.iter().map(|row| {
        let mut test_row = *row;
        let mut n = 0;
        while test_row.any() {
            test_row >>= 1;
            n += 1;
        }
        n
    }).max().unwrap()
}

fn is_intersect(shape: &Vec<DenseBitSet>, x: usize, y: usize, grid: &Vec<DenseBitSet>) -> bool {
    for (shape_row_num, shape_row) in shape.iter().enumerate() {
        let shifted = *shape_row << x;
        if shifted.get_bit(7) {
            return true;
        }
        if let Some(grid_row) = grid.get(shape_row_num + y) {
            if (*grid_row & shifted).any() {
                return true;
            }
        }
    }

    false
}

fn insert_shape(shape: &Vec<DenseBitSet>, x: usize, y: usize, grid: &mut Vec<DenseBitSet>) {
    for (shape_row_num, shape_row) in shape.iter().enumerate() {
        let shifted = *shape_row << x;
        if let Some(grid_row) = grid.get_mut(shape_row_num + y) {
            *grid_row |= shifted;
        } else {
            grid.push(shifted);
        }
    }
}

fn display_grid(grid: &Vec<DenseBitSet>) {
    for row in grid.iter().rev() {
        let row_str = (0..7).rev().map(|i| match row.get_bit(i) {
            false => '.',
            true => '#',
        }).join("");
        println!("|{}|", &row_str);
    }
    println!("---------");
}

const ROCKS_TO_INSERT: usize = 1_000_000_000_000;

fn main() -> Result<(), Box<dyn error::Error>> {
    let pushes: Vec<Push> = fs::read_to_string(path::Path::new("./data/day17.txt"))?
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Push::Right),
            '<' => Some(Push::Left),
            '\n' => None,
            _ => panic!("Unknown character"),
        })
        .collect();

    let shapes: Vec<Vec<DenseBitSet>> = vec![
        vec![DenseBitSet::from_string("1111", 2)],
        vec![
            DenseBitSet::from_string("010", 2),
            DenseBitSet::from_string("111", 2),
            DenseBitSet::from_string("010", 2),
        ],
        // Vertically inverted!
        vec![
            DenseBitSet::from_string("111", 2),
            DenseBitSet::from_string("001", 2),
            DenseBitSet::from_string("001", 2),
        ],
        vec![
            DenseBitSet::from_string("1", 2),
            DenseBitSet::from_string("1", 2),
            DenseBitSet::from_string("1", 2),
            DenseBitSet::from_string("1", 2),
        ],
        vec![
            DenseBitSet::from_string("11", 2),
            DenseBitSet::from_string("11", 2),
        ],
    ];

    let mut grid: Vec<DenseBitSet> = Vec::new();

    let mut shape_idx = 0;
    let mut push_idx = 0;
    for idx in 0..ROCKS_TO_INSERT {
        if idx % 100000 == 0 {
            println!("{}", (idx as f32)/(ROCKS_TO_INSERT as f32));
        }

        let shape = &shapes[shape_idx % shapes.len()];
        let mut x: usize = 7 - (2 + get_shape_width(shape));
        let mut y: usize = get_grid_height(&grid) + 3;

        if is_intersect(shape, x, y, &grid) {
            panic!("Intersection at start");
        }

        loop {
            let push = &pushes[push_idx % pushes.len()];
            let maybe_new_x = match push {
                Push::Right => {
                    if x > 0 {
                        Some(x - 1)
                    } else {
                        None
                    }
                }
                Push::Left => Some(x + 1),
            };

            if let Some(new_x) = maybe_new_x && !is_intersect(shape, new_x, y, &grid) {
                x = new_x;
            }
            push_idx += 1;

            if y == 0 || is_intersect(shape, x, y - 1, &grid) {
                break;
            }
            y -= 1;
        }

        insert_shape(shape, x, y, &mut grid);
        shape_idx += 1;

        //println!();
        //println!();
        //display_grid(&grid);
    }
    
    dbg!(get_grid_height(&grid));

    Ok(())
}
