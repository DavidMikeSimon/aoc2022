use std::{error, fs, path, io::{self, BufRead}, convert::TryInto, collections::{HashSet, HashMap}, iter};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cell {
    height: usize,
    distance: usize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day12.txt"))?;
    let mut start:  Option<Point> = None;
    let mut end:  Option<Point> = None;
    let mut grid: Vec<Vec<Cell>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().map(|(col, c)| {
                if c == 'S' {
                    start = Some(Point { x: col, y: row });
                }
                if c == 'E' {
                    end = Some(Point { x: col, y: row });
                }
                Cell {
                    height: match c {
                        'S' => 1usize,
                        'E' => 26usize,
                        _ => c.to_digit(36).unwrap() as usize - 10usize,
                    },
                    distance: 99999,
                }
            }).collect()
        }).collect();
    
    let start = start.ok_or("No start point")?;
    let end = end.ok_or("No end point")?;

    // grid[start.y][start.x].distance = 0;
    grid[end.y][end.x].distance = 0;

    let offsets: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    loop {
        let mut distances_to_write: HashMap<Point, usize> = HashMap::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let cell = &grid[y][x];
                for (x_offset, y_offset) in offsets.iter() {
                    let tgt_x = x as isize + x_offset;
                    let tgt_y = y as isize + y_offset;
                    if tgt_x < 0 || tgt_x >= grid[y].len() as isize || tgt_y < 0 || tgt_y >= grid.len() as isize  {
                        continue;
                    }

                    let tgt_x = tgt_x as usize;
                    let tgt_y = tgt_y as usize;
                    let tgt_cell = &grid[tgt_y][tgt_x];

                    // if tgt_cell.height > cell.height+1 {
                    //     continue;
                    // }
                    if cell.height == 0 || tgt_cell.height < cell.height-1 {
                        continue;
                    }

                    let new_dist = cell.distance + 1;
                    if tgt_cell.distance <= new_dist {
                        continue;
                    }

                    distances_to_write
                        .entry(Point{x: tgt_x, y: tgt_y})
                        .and_modify(|d| *d = (new_dist).min(*d) )
                        .or_insert(new_dist);
                }
            }
        }

        if distances_to_write.is_empty() {
            println!("No writes");
            break;
        } else if let Some(end_distance) = distances_to_write.get(&end) {
            println!("Final distance: {}", end_distance);
            break;
        } else {
            dbg!(distances_to_write.len());
            for (point, distance) in distances_to_write.drain() {
                grid[point.y][point.x].distance = distance;
            }
        }
    }

    let m = &grid.iter().flat_map(|row|
        row.iter().filter(|cell| cell.height == 0).map(|cell| cell.distance)
    ).min();
    dbg!(m);

    Ok(())
}
