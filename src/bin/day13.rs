use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter, path,
};

use itertools::{EitherOrBoth, Itertools};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Ord, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Scalar(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Scalar(a), Packet::Scalar(b)) => a.partial_cmp(b),
            (Packet::List(a_items), Packet::List(b_items)) => {
                for either_or_both in a_items.iter().zip_longest(b_items.iter()) {
                    match either_or_both {
                        itertools::EitherOrBoth::Both(a, b) => {
                            let a_vs_b = a.partial_cmp(b);
                            if a_vs_b != Some(Ordering::Equal) {
                                return a_vs_b;
                            }
                        }
                        itertools::EitherOrBoth::Right(_) => {
                            return Some(Ordering::Less);
                        }
                        itertools::EitherOrBoth::Left(_) => {
                            return Some(Ordering::Greater);
                        }
                    }
                }
                return Some(Ordering::Equal);
            }
            (Packet::Scalar(a), b_list) => {
                Packet::List(vec![Packet::Scalar(*a)]).partial_cmp(b_list)
            }
            (a_list, Packet::Scalar(b)) => {
                a_list.partial_cmp(&Packet::List(vec![Packet::Scalar(*b)]))
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(path::Path::new("./data/day13.txt"))?;
    let pairs: Vec<(Packet, Packet)> = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?
        .chunks(3)
        .map(|chunk| {
            (
                serde_json::from_str(&chunk[0]).unwrap(),
                serde_json::from_str(&chunk[1]).unwrap(),
            )
        })
        .collect();

    let ordered_indices: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (packet_a, packet_b))| {
            if packet_a > packet_b {
                None
            } else {
                Some(idx + 1)
            }
        })
        .sum();

    dbg!(&ordered_indices);

    let mut packets: Vec<Packet> = pairs
        .iter()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect();

    let two = Packet::List(vec![Packet::Scalar(2)]);
    let six = Packet::List(vec![Packet::Scalar(6)]);
    packets.push(two.clone());
    packets.push(six.clone());
    packets.sort();

    let (two_pos, _) = packets.iter().find_position(|&p| p == &two).unwrap();
    let (six_pos, _) = packets.iter().find_position(|&p| p == &six).unwrap();
    dbg!((two_pos + 1) * (six_pos + 1));

    Ok(())
}
