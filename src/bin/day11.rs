use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

#[derive(Debug)]
enum Op {
    AddConstant(usize),
    MulConstant(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test_divisor: usize,
    true_tgt_idx: usize,
    false_tgt_idx: usize,
    inspections: usize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let file = fs::File::open(path::Path::new("./data/day11.txt"))?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect();

    for monkey_lines in lines.chunks(7) {
        let (_, items_str) = monkey_lines[1].split_at(16);
        let (_, op_str) = monkey_lines[2].split_at(21);
        let (_, operand_str) = monkey_lines[2].split_at(23);
        let (_, div_str) = monkey_lines[3].split_at(19);
        let (_, true_tgt_str) = monkey_lines[4].split_at(25);
        let (_, false_tgt_str) = monkey_lines[5].split_at(26);

        monkeys.push(Monkey {
            items: items_str.split(", ").map(|s| s.parse().unwrap()).collect(),
            op: match op_str.chars().next().unwrap() {
                '+' => Op::AddConstant(operand_str.parse().unwrap()),
                '*' => {
                    if operand_str.chars().next().unwrap() == 'o' {
                        Op::Square
                    } else {
                        Op::MulConstant(operand_str.parse().unwrap())
                    }
                }
                _ => panic!("Unknown operation {}", op_str),
            },
            test_divisor: div_str.parse().unwrap(),
            true_tgt_idx: true_tgt_str.parse().unwrap(),
            false_tgt_idx: false_tgt_str.parse().unwrap(),
            inspections: 0,
        });
    }

    let combined_primes: usize = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut move_targets: HashMap<usize, Vec<usize>> = HashMap::new();
            {
                let monkey = &mut monkeys[i];
                monkey.inspections += monkey.items.len();
                for item in monkey.items.drain(0..) {
                    let new_value = match monkey.op {
                        Op::AddConstant(c) => item + c,
                        Op::MulConstant(c) => item * c,
                        Op::Square => item * item,
                    };
                    let new_value = new_value % combined_primes;
                    let target_idx = if new_value % monkey.test_divisor == 0 {
                        monkey.true_tgt_idx
                    } else {
                        monkey.false_tgt_idx
                    };
                    move_targets
                        .entry(target_idx)
                        .and_modify(|v| v.push(new_value))
                        .or_insert_with(|| vec![new_value]);
                }
            }

            for (&tgt_idx, moved_items) in move_targets.iter_mut() {
                monkeys[tgt_idx].items.append(moved_items);
            }
        }
    }

    dbg!(&monkeys);

    Ok(())
}
