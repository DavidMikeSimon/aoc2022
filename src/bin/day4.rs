use std::{error, fs, path, io};
use std::io::BufRead;
use regex::Regex;

fn main() -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    {
        let file = fs::File::open(path::Path::new("./data/day4.txt"))?;
        let result: usize = io::BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .into_iter()
            .map(|line| {
                let line = line.trim();
                if line.len() == 0 {
                    return Ok(0);
                }

                let caps = re.captures(line).unwrap();
                let (startA, endA, startB, endB) = (
                    caps.get(1).unwrap().as_str().parse::<usize>()?,
                    caps.get(2).unwrap().as_str().parse::<usize>()?,
                    caps.get(3).unwrap().as_str().parse::<usize>()?,
                    caps.get(4).unwrap().as_str().parse::<usize>()?,
                );

                dbg!((startA, endA));
                dbg!((startB, endB));

                // if startA <= startB && endA >= endB {
                    // return dbg!(Ok(1));
                // }

                // if startB <= startA && endB >= endA {
                    // return dbg!(Ok(1));
                // }

                if startA >= startB && startA <= endB {
                    return dbg!(Ok(1));
                }

                if startB >= startA && startB <= endA {
                    return dbg!(Ok(1));
                }

                dbg!(Ok(0))
            })
            .collect::<Result<Vec<usize>, Box<dyn error::Error>>>()?
            .into_iter()
            .sum();
        println!("{}", result);
    }

    Ok(())
}
