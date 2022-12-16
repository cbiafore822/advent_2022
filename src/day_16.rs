use regex::Regex;

use crate::get_input;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_16.txt";
const TEST: &str = "inputs/test.txt";

pub fn find_maximal_pressure() -> Result<usize> {
    let input = get_input(INPUT)?;
    let valves: HashMap<String, Valve> = input
        .lines()
        .into_iter()
        .map(|line| {
            let valve = Valve::new(line.to_string());
            (valve.name.clone(), valve)
        })
        .collect();
    let mut max_pressure = 0;
    let mut paths = Vec::from([State::new(
        0,
        "AA".to_string(),
        "".to_string(),
        30,
        HashSet::new(),
    )]);
    while !paths.is_empty() {
        paths = paths
            .iter()
            .map(|state| {
                let new_states = state.find_paths(&valves);
                for new_state in &new_states {
                    max_pressure = max(max_pressure, new_state.total_pressure);
                }
                new_states
            })
            .flatten()
            .collect();
    }
    Ok(max_pressure)
}

pub fn find_maximal_pressure_with_elephant() -> Result<usize> {
    let input = get_input(TEST)?;
    let valves: HashMap<String, Valve> = input
        .lines()
        .into_iter()
        .map(|line| {
            let valve = Valve::new(line.to_string());
            (valve.name.clone(), valve)
        })
        .collect();
    let mut max_pressure = 0;
    let mut paths = Vec::from([State::new(
        0,
        "AA".to_string(),
        "AA".to_string(),
        26,
        HashSet::new(),
    )]);
    while !paths.is_empty() {
        paths = paths
            .iter()
            .map(|state| {
                let new_states = state.find_paths_with_elephant(&valves);
                for new_state in &new_states {
                    max_pressure = max(max_pressure, new_state.total_pressure);
                }
                new_states
            })
            .flatten()
            .collect();
    }
    Ok(max_pressure)
}

struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(input: String) -> Self {
        let re = Regex::new(r"\d+|[A-Z]{2}").unwrap();
        let mut info = re.find_iter(&input);
        let name = info.next().unwrap().as_str().to_string();
        let rate = info.next().unwrap().as_str().parse().unwrap();
        let mut tunnels = Vec::new();
        while let Some(tunnel) = info.next() {
            tunnels.push(tunnel.as_str().to_string());
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
    e: String,
    time: usize,
    on: HashSet<String>,
}

impl State {
    fn new(
        total_pressure: usize,
        curr: String,
        e: String,
        time: usize,
        on: HashSet<String>,
    ) -> Self {
        State {
            total_pressure,
            curr,
            e,
            time,
            on,
        }
    }

    fn find_paths(&self, valves: &HashMap<String, Valve>) -> Vec<Self> {
        let mut pressure_time = self.time - 1;
        let mut queue = Vec::from([&self.curr]);
        let mut visited = HashSet::from([&self.curr]);
        let mut paths = Vec::new();
        while pressure_time > 0 && !queue.is_empty() {
            queue = queue
                .iter()
                .map(|curr| {
                    let curr_str = curr.clone();
                    let curr_valve = valves.get(curr_str).unwrap();
                    if curr_valve.rate != 0 && !self.on.contains(curr_str) {
                        let mut new_state = State::new(
                            self.total_pressure + curr_valve.rate * pressure_time,
                            curr_str.clone(),
                            "".to_string(),
                            pressure_time,
                            self.on.clone(),
                        );
                        new_state.on.insert(curr_str.to_string());
                        paths.push(new_state)
                    }
                    curr_valve
                        .tunnels
                        .iter()
                        .filter_map(|tunnel| {
                            if visited.insert(tunnel) {
                                Some(tunnel)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<&String>>()
                })
                .flatten()
                .collect();
            pressure_time -= 1;
        }
        paths
    }

    fn find_paths_with_elephant(&self, valves: &HashMap<String, Valve>) -> Vec<Self> {
        let mut pressure_time = self.time - 1;
        let mut queue = Vec::from([(&self.curr, &self.e)]);
        let mut visited = HashSet::from([(&self.curr, &self.e)]);
        let mut paths = Vec::new();
        while pressure_time > 0 && !queue.is_empty() {
            queue = queue
                .iter()
                .map(|curr| {
                    let (curr_str, e_str) = (curr.0, curr.1);
                    let (curr_valve, e_valve) =
                        (valves.get(curr_str).unwrap(), valves.get(e_str).unwrap());
                    let mut next = curr_valve
                        .tunnels
                        .iter()
                        .map(|tunnel_c| {
                            let mut temp = e_valve
                                .tunnels
                                .iter()
                                .filter_map(|tunnel_e| {
                                    if visited.insert((tunnel_c, tunnel_e)) {
                                        Some((tunnel_c, tunnel_e))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<(&String, &String)>>();
                            if visited.insert((tunnel_c, &e_valve.name)) {
                                temp.push((tunnel_c, &e_valve.name));
                            }
                            temp
                        })
                        .flatten()
                        .collect::<Vec<(&String, &String)>>();
                    next.append(
                        &mut e_valve
                            .tunnels
                            .iter()
                            .filter_map(|tunnel_e| {
                                if visited.insert((&curr_valve.name, tunnel_e)) {
                                    Some((&curr_valve.name, tunnel_e))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<(&String, &String)>>(),
                    );
                    if (curr_valve.rate != 0 || e_valve.rate != 0)
                        && !self.on.contains(curr_str)
                        && !self.on.contains(e_str)
                    {
                        let mut new_state = State::new(
                            self.total_pressure
                                + curr_valve.rate * pressure_time
                                + e_valve.rate * pressure_time,
                            curr_str.clone(),
                            e_str.clone(),
                            pressure_time,
                            self.on.clone(),
                        );
                        new_state.on.insert(curr_str.to_string());
                        new_state.on.insert(e_str.to_string());
                        paths.push(new_state)
                    }
                    next
                })
                .flatten()
                .collect();
            pressure_time -= 1;
        }
        paths
    }
}
