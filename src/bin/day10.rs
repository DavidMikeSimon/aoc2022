use std::{
    collections::HashSet,
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut cycle = 0;
    let mut x_reg = 1;
    let mut score = 0;

    let file = fs::File::open(path::Path::new("./data/day10.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let mut items = line.split_ascii_whitespace();
        let instruction = items.next().unwrap();
        let instruction_cycles = match instruction {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Unknown instruction {}", instruction),
        };

        for _ in 0..instruction_cycles {
            let h_pos: i32 = cycle % 40;

            if (h_pos - x_reg).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }

            if cycle > 0 && h_pos == 39 {
                println!();
            }

            cycle += 1;

            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                score += cycle * x_reg;
            }
        }

        if instruction == "addx" {
            let arg: i32 = items.next().unwrap().parse()?;
            x_reg += arg;
        }
    }

    //dbg!(score);

    Ok(())
}
