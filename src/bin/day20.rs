use std::{
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    path,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    value: isize,
    original_position: isize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day20.txt"))?;
    let mut items: Vec<Item> = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line = line.unwrap();
            let line = line.trim();
            Item {
                value: line.parse::<isize>().unwrap() * 811589153,
                original_position: idx as isize,
            }
        })
        .collect();

    let full_len = items.len();
    dbg!(full_len);

    for _ in 0..10 {
        for idx in 0..full_len {
            let (old_pos, _) = items
                .iter()
                .find_position(|item| item.original_position == idx.try_into().unwrap())
                .unwrap();

            let item = items.remove(old_pos);
            let mut new_pos = (old_pos as isize) + item.value;
            if new_pos <= 0 {
                let x = (-new_pos) / ((full_len as isize) - 1) + 1;
                new_pos += ((full_len as isize) - 1) * x;
            }
            if new_pos > full_len as isize {
                let x = new_pos / ((full_len as isize) - 1);
                new_pos -= ((full_len as isize) - 1) * x;
            }
            items.insert(new_pos as usize, item);
        }
    }

    let mut n: isize = 0;
    let (zero_pos, _) = items.iter().find_position(|i| i.value == 0).unwrap();
    n += items[(1000 + zero_pos) % items.len()].value;
    n += items[(2000 + zero_pos) % items.len()].value;
    n += items[(3000 + zero_pos) % items.len()].value;
    dbg!(n);

    Ok(())
}
