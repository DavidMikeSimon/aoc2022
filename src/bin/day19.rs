use std::{
    error, fs,
    io::{self, BufRead},
    path,
};

use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct Blueprint {
    id: u16,
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,
    maximum_ore_cost: u16,
}

#[derive(Default, Debug, Clone)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    open_geodes: u16,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl State {
    fn new() -> State {
        State {
            ore_robots: 1,
            ..State::default()
        }
    }

    fn wait(&self) -> State {
        State {
            ore: self.ore + self.ore_robots as u16,
            clay: self.clay + self.clay_robots as u16,
            obsidian: self.obsidian + self.obsidian_robots as u16,
            open_geodes: self.open_geodes + self.geode_robots as u16,
            ..*self
        }
    }

    fn build_ore_robot(&self, state_after_wait: &State, blueprint: &Blueprint) -> Option<State> {
        if self.ore < blueprint.ore_robot_ore_cost {
            return None;
        }

        Some(State {
            ore_robots: self.ore_robots + 1,
            ore: state_after_wait.ore - blueprint.ore_robot_ore_cost,
            ..*state_after_wait
        })
    }

    fn build_clay_robot(&self, state_after_wait: &State, blueprint: &Blueprint) -> Option<State> {
        if self.ore < blueprint.clay_robot_ore_cost {
            return None;
        }

        Some(State {
            clay_robots: self.clay_robots + 1,
            ore: state_after_wait.ore - blueprint.clay_robot_ore_cost,
            ..*state_after_wait
        })
    }

    fn build_obsidian_robot(
        &self,
        state_after_wait: &State,
        blueprint: &Blueprint,
    ) -> Option<State> {
        if self.ore < blueprint.obsidian_robot_ore_cost {
            return None;
        }
        if self.clay < blueprint.obsidian_robot_clay_cost {
            return None;
        }

        Some(State {
            obsidian_robots: self.obsidian_robots + 1,
            ore: state_after_wait.ore - blueprint.obsidian_robot_ore_cost,
            clay: state_after_wait.clay - blueprint.obsidian_robot_clay_cost,
            ..*state_after_wait
        })
    }

    fn build_geode_robot(&self, state_after_wait: &State, blueprint: &Blueprint) -> Option<State> {
        if self.ore < blueprint.geode_robot_ore_cost {
            return None;
        }
        if self.obsidian < blueprint.geode_robot_obsidian_cost {
            return None;
        }

        Some(State {
            geode_robots: self.geode_robots + 1,
            ore: state_after_wait.ore - blueprint.geode_robot_ore_cost,
            obsidian: state_after_wait.obsidian - blueprint.geode_robot_obsidian_cost,
            ..*state_after_wait
        })
    }
}

fn find_maximum_geodes(state: &State, blueprint: &Blueprint, minutes_remaining: usize) -> usize {
    if minutes_remaining == 0 {
        return state.open_geodes as usize;
    }

    let mut best_score: usize = 0;

    let state_after_wait = state.wait();

    if let Some(geode_robot_state) = state.build_geode_robot(&state_after_wait, blueprint) {
        best_score = best_score.max(find_maximum_geodes(
            &geode_robot_state,
            blueprint,
            minutes_remaining - 1,
        ));
    } else {
        // If we can't build a geode robot, try just waiting
        best_score = best_score.max(find_maximum_geodes(
            &state_after_wait,
            blueprint,
            minutes_remaining - 1,
        ));
    }

    if (state.obsidian_robots as u16) < blueprint.geode_robot_obsidian_cost {
        if let Some(obsidian_robot_state) = state.build_obsidian_robot(&state_after_wait, blueprint)
        {
            best_score = best_score.max(find_maximum_geodes(
                &obsidian_robot_state,
                blueprint,
                minutes_remaining - 1,
            ));
        }

        if (state.clay_robots as u16) < blueprint.obsidian_robot_clay_cost {
            if let Some(clay_robot_state) = state.build_clay_robot(&state_after_wait, blueprint) {
                best_score = best_score.max(find_maximum_geodes(
                    &clay_robot_state,
                    blueprint,
                    minutes_remaining - 1,
                ));
            }
        }
    }

    if (state.ore_robots as u16) < blueprint.maximum_ore_cost {
        if let Some(ore_robot_state) = state.build_ore_robot(&state_after_wait, blueprint) {
            best_score = best_score.max(find_maximum_geodes(
                &ore_robot_state,
                blueprint,
                minutes_remaining - 1,
            ));
        }
    }

    best_score
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    )?;
    let file = fs::File::open(path::Path::new("./data/day19.txt"))?;
    let blueprints: Vec<Blueprint> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let cap = re.captures(&line).unwrap();

            let id: u16 = cap.get(1).unwrap().as_str().parse().unwrap();
            let ore_robot_ore_cost: u16 = cap.get(2).unwrap().as_str().parse().unwrap();
            let clay_robot_ore_cost: u16 = cap.get(3).unwrap().as_str().parse().unwrap();
            let obsidian_robot_ore_cost: u16 = cap.get(4).unwrap().as_str().parse().unwrap();
            let obsidian_robot_clay_cost: u16 = cap.get(5).unwrap().as_str().parse().unwrap();
            let geode_robot_ore_cost: u16 = cap.get(6).unwrap().as_str().parse().unwrap();
            let geode_robot_obsidian_cost: u16 = cap.get(7).unwrap().as_str().parse().unwrap();

            Blueprint {
                id,
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost,
                maximum_ore_cost: ore_robot_ore_cost
                    .max(clay_robot_ore_cost)
                    .max(obsidian_robot_ore_cost)
                    .max(geode_robot_ore_cost),
            }
        })
        .collect();

    let score: usize = blueprints
        .par_iter()
        .map(|blueprint| {
            find_maximum_geodes(&State::new(), blueprint, 24) * (blueprint.id as usize)
        })
        .sum();

    // let score: usize = blueprints
    //     .par_iter()
    //     .take(3)
    //     .map(|blueprint| {
    //         find_maximum_geodes(&State::new(), blueprint, 32)
    //     })
    //     .product();

    dbg!(score);

    Ok(())
}
