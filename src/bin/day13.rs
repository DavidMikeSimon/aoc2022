use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day13.txt"))?;
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .enumerate();

    Ok(())
}
