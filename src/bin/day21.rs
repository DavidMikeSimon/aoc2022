use std::{
    collections::HashMap,
    convert::TryInto,
    error,
    fmt::{Debug, Write},
    fs,
    io::{self, BufRead},
    path,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Name([char; 4]);

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}{}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        ))
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name([
            value.chars().nth(0).unwrap(),
            value.chars().nth(1).unwrap(),
            value.chars().nth(2).unwrap(),
            value.chars().nth(3).unwrap(),
        ])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eql,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Expr {
    Constant(isize),
    BinaryExpr(BinOp, Name, Name),
    HumanInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    expr: Expr,
    value: Option<isize>,
}

fn solve(monkeys: &HashMap<Name, Monkey>, op: BinOp, a: Name, b: Name, expected: isize) -> isize {
    let monkey_a = monkeys.get(&a).unwrap();
    let monkey_b = monkeys.get(&b).unwrap();

    let (solved, unsolved, is_solved_first) = match (monkey_a, monkey_b) {
        (
            Monkey {
                value: Some(n),
                expr: _,
            },
            _,
        ) => (n, monkey_b, true),
        (
            _,
            Monkey {
                value: Some(n),
                expr: _,
            },
        ) => (n, monkey_a, false),
        _ => panic!("Neither monkey is solved"),
    };

    let other = match (op, is_solved_first) {
        (BinOp::Add, _) => expected - solved,
        (BinOp::Sub, true) => solved - expected,
        (BinOp::Sub, false) => solved + expected,
        (BinOp::Mul, _) => expected / solved,
        (BinOp::Div, true) => solved / expected,
        (BinOp::Div, false) => solved * expected,
        (BinOp::Eql, _) => panic!("Eql operator???"),
    };

    match unsolved.expr {
        Expr::HumanInput => other,
        Expr::Constant(_) => panic!("Unsolved constant???"),
        Expr::BinaryExpr(unsolved_op, unsolved_a, unsolved_b) => {
            solve(monkeys, unsolved_op, unsolved_a, unsolved_b, other)
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let re =
        Regex::new(r"(?P<name>\w+): ((?P<constant>\d+)|(?P<arg1>\w+) (?P<op>.) (?P<arg2>\w+))")?;

    let file = fs::File::open(path::Path::new("./data/day21.txt"))?;
    let mut monkeys: HashMap<Name, Monkey> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let cap = re.captures(&line).unwrap();

            let name: Name = cap.name("name").unwrap().as_str().into();
            if name == "humn".into() {
                return (
                    name,
                    Monkey {
                        expr: Expr::HumanInput,
                        value: None,
                    },
                );
            }

            let monkey = match cap.name("constant") {
                Some(constant_group) => Monkey {
                    expr: Expr::Constant(constant_group.as_str().parse().unwrap()),
                    value: None,
                },
                None => {
                    let op = match name == "root".into() {
                        true => BinOp::Eql,
                        false => match cap.name("op").unwrap().as_str().chars().nth(0).unwrap() {
                            '+' => BinOp::Add,
                            '-' => BinOp::Sub,
                            '*' => BinOp::Mul,
                            '/' => BinOp::Div,
                            _ => panic!("Unrecognized operator"),
                        },
                    };
                    let arg1: Name = cap.name("arg1").unwrap().as_str().into();
                    let arg2: Name = cap.name("arg2").unwrap().as_str().into();
                    Monkey {
                        expr: Expr::BinaryExpr(op, arg1, arg2),
                        value: None,
                    }
                }
            };
            (name, monkey)
        })
        .collect();

    loop {
        let updates: HashMap<Name, Monkey> = monkeys
            .iter()
            .filter_map(|(name, monkey)| {
                if monkey.value.is_some() {
                    return None;
                }

                match monkey.expr {
                    Expr::Constant(n) => Some((
                        *name,
                        Monkey {
                            expr: monkey.expr,
                            value: Some(n),
                        },
                    )),
                    Expr::BinaryExpr(op, arg1, arg2) => {
                        match (
                            monkeys.get(&arg1).unwrap().value,
                            monkeys.get(&arg2).unwrap().value,
                        ) {
                            (Some(val1), Some(val2)) => {
                                let value = match op {
                                    BinOp::Add => val1 + val2,
                                    BinOp::Sub => val1 - val2,
                                    BinOp::Mul => val1 * val2,
                                    BinOp::Div => val1 / val2,
                                    _ => return None,
                                };
                                Some((
                                    *name,
                                    Monkey {
                                        expr: monkey.expr,
                                        value: Some(value),
                                    },
                                ))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        if updates.len() == 0 {
            break;
        } else {
            monkeys.extend(updates.iter());
        }
    }

    let root = monkeys.get(&"root".into()).unwrap();
    let (solved, unsolved) = if let Expr::BinaryExpr(BinOp::Eql, name_a, name_b) = root.expr {
        let a = monkeys.get(&name_a).unwrap();
        let b = monkeys.get(&name_b).unwrap();
        if a.value.is_some() {
            (a.value.unwrap(), b)
        } else {
            (b.value.unwrap(), a)
        }
    } else {
        panic!("Cannot find root equality expression");
    };

    if let Expr::BinaryExpr(op, arg1, arg2) = unsolved.expr {
        dbg!(solve(&monkeys, op, arg1, arg2, solved));
    } else {
        panic!("Not binary expr on unsolved");
    }

    Ok(())
}
