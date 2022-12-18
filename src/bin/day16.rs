#![feature(drain_filter)]
#![feature(let_chains)]

use core::time;
use itertools::{Itertools, MinMaxResult};
use regex::Regex;
use std::{
    cmp::{min, Ordering},
    collections::{btree_map::Range, HashMap, HashSet},
    convert::TryInto,
    error, fs,
    io::{self, BufRead},
    iter,
    ops::RangeInclusive,
    path,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Name {
    c1: char,
    c2: char,
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = vec![self.c1, self.c2].into_iter().collect();
        f.write_str(&s)
    }
}

impl Name {
    fn new(s: &str) -> Name {
        Name {
            c1: s.chars().nth(0).unwrap(),
            c2: s.chars().nth(1).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Valve {
    name: Name,
    flow_rate: usize,
    tunnels: HashSet<Name>,
}

fn get_distance(
    valves: &HashMap<Name, Valve>,
    start: &Name,
    end: &Name,
    visited: &HashSet<Name>,
) -> usize {
    if start == end {
        return 0;
    }

    let start_valve = valves.get(start).unwrap();
    start_valve
        .tunnels
        .iter()
        .filter_map(|dest_name| {
            if visited.contains(dest_name) {
                return None;
            }

            let mut new_visited = visited.clone();
            new_visited.insert(start.to_owned());
            Some(get_distance(valves, dest_name, end, &new_visited) + 1)
        })
        .min()
        .unwrap_or(9999999)
}

fn get_flow_valve_distances(
    initial_start: &Name,
    valves: &HashMap<Name, Valve>,
) -> HashMap<Name, HashMap<Name, usize>> {
    let flow_valves: Vec<Name> = valves
        .iter()
        .filter_map(|(_, valve)| {
            if valve.name == *initial_start || valve.flow_rate > 0 {
                Some(valve.name)
            } else {
                None
            }
        })
        .sorted()
        .collect();

    let mut distances: HashMap<Name, HashMap<Name, usize>> = HashMap::new();

    for (start_idx, &start) in flow_valves.iter().enumerate() {
        for &end in &flow_valves[start_idx + 1..] {
            let distance = get_distance(&valves, &start, &end, &HashSet::new());
            let from_start_map = distances.entry(start).or_default();
            from_start_map.insert(end, distance);
            let from_end_map = distances.entry(end).or_default();
            from_end_map.insert(start, distance);
        }
    }

    distances
}

fn best_flow(
    current_valve: &Name,
    valves: &HashMap<Name, Valve>,
    target_valves: &Vec<Name>,
    flow_valve_distances: &HashMap<Name, HashMap<Name, usize>>,
    remaining_minutes: usize,
    activated_valves: &HashSet<Name>,
) -> usize {
    if remaining_minutes == 0 {
        return 0;
    }

    let minute_flow: usize = activated_valves
        .iter()
        .map(|name| valves.get(name).unwrap().flow_rate)
        .sum();

    let wait_outcome = remaining_minutes * minute_flow;

    let distances_from_here = flow_valve_distances.get(current_valve).unwrap();
    let maybe_movement_outcome = target_valves
        .iter()
        .filter_map(|dest_name| {
            if activated_valves.contains(dest_name) {
                return None;
            }

            let distance = distances_from_here.get(dest_name).unwrap();
            if *distance < remaining_minutes {
                let time_elapsed = distance + 1;
                let mut new_activated_valves = activated_valves.clone();
                new_activated_valves.insert(dest_name.to_owned());
                Some(
                    best_flow(
                        dest_name,
                        valves,
                        target_valves,
                        flow_valve_distances,
                        remaining_minutes - time_elapsed,
                        &new_activated_valves,
                    ) + time_elapsed * minute_flow,
                )
            } else {
                None
            }
        })
        .max();

    if let Some(n) = maybe_movement_outcome && n > wait_outcome {
        n
    } else {
        wait_outcome
    }
}

fn all_partitions(valve_names: &[Name]) -> Vec<(Vec<Name>, Vec<Name>)> {
    if valve_names.len() == 0 {
        return vec![];
    }

    let head = &valve_names[0];

    if valve_names.len() == 1 {
        return vec![(vec![*head], vec![]), (vec![], vec![*head])];
    }

    let rest_partitions = all_partitions(&valve_names[1..]);
    rest_partitions
        .iter()
        .flat_map(|(a, b)| {
            let mut extended_a = a.clone();
            extended_a.push(*head);
            let mut extended_b = b.clone();
            extended_b.push(*head);
            vec![(extended_a, b.clone()), (a.clone(), extended_b)]
        })
        .collect()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)")?;

    let mut valves: HashMap<Name, Valve> = HashMap::new();

    let file = fs::File::open(path::Path::new("./data/day16.txt"))?;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let cap = re
            .captures(&line)
            .ok_or_else(|| format!("Couldn't parse line: {}", &line))?;
        let valve = Valve {
            name: Name::new(cap.get(1).ok_or("No first capture")?.as_str()),
            flow_rate: cap.get(2).ok_or("No second capture")?.as_str().parse()?,
            tunnels: cap
                .get(3)
                .ok_or("No third capture")?
                .as_str()
                .split(",")
                .map(|s| Name::new(s.trim()))
                .collect(),
        };
        valves.insert(valve.name.clone(), valve);
    }

    let start_valve = Name::new("AA");

    let flow_valve_distances = get_flow_valve_distances(&start_valve, &valves);
    dbg!(&flow_valve_distances);

    let flow_valve_names: Vec<Name> = flow_valve_distances
        .keys()
        .filter(|n| **n != start_valve)
        .cloned()
        .collect();
    let mut partitions = all_partitions(&flow_valve_names[..]);
    partitions.sort_by_cached_key(|(a, b)| {
        let a_value: isize = a.iter().map(|x| valves[x].flow_rate as isize).sum();
        let b_value: isize = b.iter().map(|x| valves[x].flow_rate as isize).sum();
        (a_value - b_value).abs()
    });

    let mut best_score = 0;
    for (idx, (a_targets, b_targets)) in partitions.iter().enumerate() {
        if idx % 100 == 0 {
            dbg!(idx);
        }

        let score = best_flow(
            &start_valve,
            &valves,
            &a_targets,
            &flow_valve_distances,
            26,
            &HashSet::new(),
        ) + best_flow(
            &start_valve,
            &valves,
            &b_targets,
            &flow_valve_distances,
            26,
            &HashSet::new(),
        );

        if score > best_score {
            best_score = score;
            dbg!(best_score);
        }
    }

    Ok(())
}
