use regex::Regex;

use crate::get_input;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_16.txt";
const TEST: &str = "inputs/test.txt";

pub fn find_max_pressure() -> Result<usize> {
    let input = get_input(INPUT)?;
    let valves: HashMap<String, Valve> = input
        .lines()
        .into_iter()
        .map(|line| {
            let valve = Valve::from(line.to_string());
            (valve.name.clone(), valve)
        })
        .collect();
    let clique = make_clique(&valves);
    let mut max_pressure = 0;
    let mut paths = Vec::from([State::new(0, "AA".to_string(), 30, HashSet::new())]);
    while !paths.is_empty() {
        paths = paths
            .iter()
            .map(|state| {
                let new_states = state.get_next_states(&clique);
                if new_states.len() == 0 {
                    max_pressure = max(max_pressure, state.total_pressure);
                }
                new_states
            })
            .flatten()
            .collect();
    }
    Ok(max_pressure)
}

pub fn find_max_pressure_with_elephant() -> Result<usize> {
    let input = get_input(TEST)?;
    let valves: HashMap<String, Valve> = input
        .lines()
        .into_iter()
        .map(|line| {
            let valve = Valve::from(line.to_string());
            (valve.name.clone(), valve)
        })
        .collect();
    let clique = make_clique(&valves);
    let mut paths = Vec::from([State::new(0, "AA".to_string(), 30, HashSet::new())]);
    let mut possible_paths = Vec::new();
    while !paths.is_empty() {
        paths = paths
            .iter()
            .map(|state| {
                let new_states = state.get_next_states(&clique);
                if new_states.len() == 0 {
                    possible_paths.push((state.total_pressure, state.on.clone()));
                }
                new_states
            })
            .flatten()
            .collect();
    }
    let mut max_pressure = 0;
    for i in 0..possible_paths.len() {
        let (p_pressure, p_on) = &possible_paths[i];
        for j in (i + 1)..possible_paths.len() {
            let (e_pressure, e_on) = &possible_paths[j];
            if p_pressure + e_pressure > max_pressure && p_on.intersection(&e_on).count() == 0 {
                max_pressure = p_pressure + e_pressure;
            }
        }
    }
    Ok(max_pressure)
}

fn make_clique(valves: &HashMap<String, Valve>) -> HashMap<String, Valve> {
    let mut clique = HashMap::new();
    for (name, valve) in valves {
        if valve.rate == 0 && name.as_str() != "AA" {
            continue;
        }
        let mut tunnels = Vec::new();
        let mut queue = Vec::from([name.clone()]);
        let mut visited = HashSet::from([name.clone()]);
        let mut dist = 0;
        while !queue.is_empty() {
            queue = queue
                .iter()
                .map(|curr| {
                    let valve = valves.get(curr).unwrap();
                    if valve.rate != 0 && curr != name {
                        tunnels.push((curr.to_string(), dist + 1));
                    }
                    valve
                        .tunnels
                        .iter()
                        .filter_map(|tunnel| {
                            if visited.insert(tunnel.0.clone()) {
                                Some(tunnel.0.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                })
                .flatten()
                .collect();
            dist += 1;
        }
        clique.insert(name.clone(), Valve::new(name.clone(), valve.rate, tunnels));
    }
    clique
}

struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<(String, usize)>,
}

impl Valve {
    fn new(name: String, rate: usize, tunnels: Vec<(String, usize)>) -> Self {
        Valve {
            name,
            rate,
            tunnels,
        }
    }

    fn from(input: String) -> Self {
        let re = Regex::new(r"\d+|[A-Z]{2}").unwrap();
        let mut info = re.find_iter(&input);
        let name = info.next().unwrap().as_str().to_string();
        let rate = info.next().unwrap().as_str().parse().unwrap();
        let mut tunnels = Vec::new();
        while let Some(tunnel) = info.next() {
            tunnels.push((tunnel.as_str().to_string(), 1));
        }
        Valve {
            name,
            rate,
            tunnels,
        }
    }
}

struct State {
    total_pressure: usize,
    curr: String,
    time: usize,
    on: HashSet<String>,
}

impl State {
    fn new(total_pressure: usize, curr: String, time: usize, on: HashSet<String>) -> Self {
        State {
            total_pressure,
            curr,
            time,
            on,
        }
    }

    fn get_next_states(&self, valves: &HashMap<String, Valve>) -> Vec<Self> {
        let curr = valves.get(&self.curr).unwrap();
        curr.tunnels
            .iter()
            .filter_map(|tunnel| {
                let (next, dist) = tunnel;
                if self.time <= *dist || self.on.contains(next) {
                    return None;
                }
                let remaining_time = self.time - dist;
                let next_valve = valves.get(next).unwrap();
                let mut on = self.on.clone();
                on.insert(next.to_string());
                Some(State::new(
                    self.total_pressure + remaining_time * next_valve.rate,
                    next.clone(),
                    remaining_time,
                    on,
                ))
            })
            .collect()
    }
}
