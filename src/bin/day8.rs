use std::{error, fs, path, io::{self, BufRead}, convert::TryInto};

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
}

fn apply_visible<'a, T>(iter: T) where T: Iterator<Item = &'a mut Tree> {
    let mut height: i32 = -1;
    for tree in iter {
        if tree.height > height {
            tree.visible = true;
            height = tree.height;
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day8.txt"))?;
    let mut forest: Vec<Vec<Tree>> = io::BufReader::new(file).lines().map(|line| {
        line.unwrap().trim().chars().map(|c| Tree{ height: c.to_digit(10).unwrap().try_into().unwrap(), visible: false }).collect()
    }).collect();

    for row in forest.iter_mut() {
        apply_visible(row.iter_mut());
        apply_visible(row.iter_mut().rev());
    }

    for col_num in 0..forest.first().unwrap().len() {
        apply_visible(forest.iter_mut().map(|row| row.iter_mut().nth(col_num).unwrap()));
        apply_visible(forest.iter_mut().map(|row| row.iter_mut().nth(col_num).unwrap()).rev());
    }

    let visible_count: usize = forest.iter()
        .map(|row| row.iter().filter(|tree| tree.visible).count())
        .sum();
    dbg!(visible_count);

    let mut best_score: usize = 0;
    let offsets: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for row_num in 0..forest.len() as i32 {
        for col_num in 0..forest[row_num as usize].len() as i32 {
            let mut score: usize = 1;
            for offset in offsets.iter() {
                let start_height = forest[row_num as usize][col_num as usize].height;
                let mut offset_score: usize = 0;
                let mut x: i32 = col_num;
                let mut y: i32 = row_num;
                loop {
                    x += offset.0;
                    y += offset.1;
                    if x >= 0 && x < forest[0].len().try_into().unwrap() && y >= 0 && y < forest.len().try_into().unwrap() {
                        offset_score += 1;
                        if forest[y as usize][x as usize].height >= start_height {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                score *= offset_score;
            }
            if score > best_score {
                best_score = score;
            }
        }
    }

    dbg!(best_score);

	Ok(())
}
