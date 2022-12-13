use std::io::{self, BufRead};
use std::{fs, path, string};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

static CHOICES: [Choice; 3] = [Choice::Rock, Choice::Paper, Choice::Scissors];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Tie,
    Loss,
}

impl Choice {
    fn from_abc(s: char) -> Option<Choice> {
        match s {
            'A' => Some(Choice::Rock),
            'B' => Some(Choice::Paper),
            'C' => Some(Choice::Scissors),
            _ => None,
        }
    }

    fn from_xyz(s: char) -> Option<Choice> {
        match s {
            'X' => Some(Choice::Rock),
            'Y' => Some(Choice::Paper),
            'Z' => Some(Choice::Scissors),
            _ => None,
        }
    }

    fn points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn compare(&self, other: &Choice) -> Outcome {
        use Choice::*;
        use Outcome::*;

        match self {
            Rock => match other {
                Rock => Tie,
                Paper => Loss,
                Scissors => Win,
            },
            Paper => match other {
                Rock => Win,
                Paper => Tie,
                Scissors => Loss,
            },
            Scissors => match other {
                Rock => Loss,
                Paper => Win,
                Scissors => Tie,
            },
        }
    }
}

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }

    fn from_xyz(s: char) -> Option<Outcome> {
        match s {
            'X' => Some(Outcome::Loss),
            'Y' => Some(Outcome::Tie),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }

    fn my_choice_for_outcome(&self, them: &Choice) -> Choice {
        *(CHOICES
            .iter()
            .find(|&me| me.compare(them) == *self)
            .unwrap())
    }
}

fn main() {
    // Phase 1
    {
        let file = fs::File::open(path::Path::new("./data/day2.txt")).unwrap();
        let mut sum = 0;
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap();
            let line = line.trim();
            if line.len() > 0 {
                let them = Choice::from_abc(line.chars().nth(0).unwrap()).unwrap();
                let me = Choice::from_xyz(line.chars().nth(2).unwrap()).unwrap();
                let score = me.points() + me.compare(&them).points();
                sum += score;
            }
        }
        println!("{}", sum);
    }

    // Phase 2
    {
        let file = fs::File::open(path::Path::new("./data/day2.txt")).unwrap();
        let sum: usize = io::BufReader::new(file)
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let line = line.trim();
                if (line.len() == 0) {
                    return 0;
                }
                let them = Choice::from_abc(line.chars().nth(0).unwrap()).unwrap();
                let outcome = Outcome::from_xyz(line.chars().nth(2).unwrap()).unwrap();
                let me = outcome.my_choice_for_outcome(&them);
                me.points() + outcome.points()
            })
            .sum();
        println!("{}", sum);
    }
}
