use std::{fs, error, io::{self, BufRead}, path, convert::TryInto};

fn parse_snafu_digit(input: char) -> i64 {
    match input {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Unknown character {}", input),
    }
}

fn parse_snafu(input: &str) -> i64 {
    input.chars().rev().enumerate().map(|(pos, c)| {
        let digit_value = parse_snafu_digit(c);
        digit_value * 5i64.pow(pos.try_into().unwrap())
    }).sum()
}

fn encode_snafu_digit_offset(input: i64) -> char {
    match input % 5 {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => panic!("Invalid digit: {}", input)
    }
}

fn encode_snafu(input: i64) -> String {
    if input == 0 {
        return "".into();
    }

    let digit_chr = encode_snafu_digit_offset(input % 5);
    let rest = (input - parse_snafu_digit(digit_chr)) / 5;
    let mut rest_str = encode_snafu(rest);
    rest_str.push(digit_chr);
    rest_str
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day25.txt"))?;
    let numbers: Vec<i64> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            parse_snafu(line.trim())
        })
        .collect();
    
    println!("SUM: {}", encode_snafu(numbers.iter().sum()));

	Ok(())
}
