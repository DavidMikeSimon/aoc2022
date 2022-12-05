use std::{fs, path};
use std::io::{self, BufRead};

fn main() {
    let mut sums: Vec<usize> = Vec::new();
    let mut cur_sum: usize = 0;

    let file = fs::File::open(path::Path::new("./data/day1.txt")).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            sums.push(cur_sum);
            cur_sum = 0;
        } else {
            let n: usize = line.parse().unwrap();
            cur_sum += n;
        }
    }
    sums.push(cur_sum);

    sums.sort();
    let top_sums = &sums[(sums.len()-3)..];

    println!("{:?}", top_sums.iter().sum::<usize>());
}
