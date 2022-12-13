use std::{
    collections::VecDeque,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

use regex::Regex;

fn main() -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let file = fs::File::open(path::Path::new("./data/day5.txt"))?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let num_columns = (lines.first().ok_or("No first line")?.len() + 1) / 4;
    let mut columns: Vec<VecDeque<char>> = iter::repeat_with(|| VecDeque::<char>::new())
        .take(num_columns)
        .collect();

    for line in lines {
        if line.contains("[") {
            for (idx, item) in line.chars().skip(1).step_by(4).enumerate() {
                if item == ' ' {
                    continue;
                }
                columns[idx].push_front(item);
            }
            continue;
        }

        let maybe_cap = re.captures(&line);
        if let Some(cap) = maybe_cap {
            let quantity: usize = cap.get(1).ok_or("No capture group 1")?.as_str().parse()?;
            let src: usize = cap.get(2).ok_or("No capture group 2")?.as_str().parse()?;
            let dst: usize = cap.get(3).ok_or("No capture group 3")?.as_str().parse()?;
            let mut stack = VecDeque::<char>::new();
            for _ in 0..quantity {
                let item = columns
                    .get_mut(src - 1)
                    .ok_or("Bad source column index")?
                    .pop_back()
                    .ok_or("Empty source column")?;
                // columns.get_mut(dst-1).ok_or("Bad dest column")?.push_back(item);
                stack.push_front(item);
            }
            columns
                .get_mut(dst - 1)
                .ok_or("Bad dest column")?
                .append(&mut stack);
        }
    }

    for column in columns {
        print!("{}", column.back().ok_or("Empty column")?);
    }
    println!();

    Ok(())
}
