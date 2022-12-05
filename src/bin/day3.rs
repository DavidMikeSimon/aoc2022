use std::{error, fs, path, io};
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::BufRead;

fn priority(c: char) -> Result<usize, Box<dyn error::Error>> {
    let mut n: u32 = c.try_into()?;
    if n >= 97 {
        n -= 96;
    } else {
        n -= 64 - 26;
    }
    Ok(n.try_into()?)
}

fn priorities(s: &str) -> Result<HashSet<usize>, Box<dyn error::Error>> {
    Ok(s
        .chars()
        .into_iter()
        .map(priority)
        .collect::<Result<Vec<usize>, Box<dyn error::Error>>>()?
        .into_iter()
        .collect())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    {
        let file = fs::File::open(path::Path::new("./data/day3.txt"))?;
        let result: usize = io::BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .into_iter()
            .map(|line| {
                let line = line.trim();
                if line.len() == 0 {
                    return Ok(0);
                }

                let (halfA, halfB) = (&line[..line.len()/2], &line[line.len()/2..]);
                let (priA, priB) = (priorities(halfA)?, priorities(halfB)?);
                let intersect: HashSet<usize> = priA.intersection(&priB).map(|n| *n).collect();
                dbg!(intersect.into_iter().next().ok_or("Empty intersection".into()))
            })
            .collect::<Result<Vec<usize>, Box<dyn error::Error>>>()?
            .into_iter()
            .sum();
        println!("{}", result);
    }

    {
        let file = fs::File::open(path::Path::new("./data/day3.txt"))?;
        let result: usize = io::BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .chunks(3)
            .map(|chunk| {
                if chunk.len() < 3 {
                    return Ok(0);
                }

                let pri1 = priorities(chunk[0].trim())?;
                let pri2 = priorities(chunk[1].trim())?;
                let pri3 = priorities(chunk[2].trim())?;

                let intersect: HashSet<usize> = pri1.intersection(&pri2).map(|n| *n).collect();
                let intersect: HashSet<usize> = intersect.intersection(&pri3).map(|n| *n).collect();
                dbg!(intersect.into_iter().next().ok_or("Empty intersection".into()))
            })
            .collect::<Result<Vec<usize>, Box<dyn error::Error>>>()?
            .into_iter()
            .sum();
        println!("{}", result);
    }

    Ok(())
}
