use crate::get_input;
use regex::Regex;
use std::{cmp::max, collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_19.txt";
const TEST: &str = "inputs/test.txt";

const RESOURCE_LIST: [Resource; 4] = [
    Resource::Geode,
    Resource::Obsidian,
    Resource::Clay,
    Resource::Ore,
];

// Elapsed time: 5370804 us
// Memory Used: 50118.02 kb
pub fn get_quality_levels() -> Result<isize> {
    let blueprints = Blueprint::from(get_input(INPUT)?);
    let mut quality_level = 0;
    for blueprint in blueprints {
        quality_level += blueprint.simulate_blueprint(24) * blueprint.number;
    }
    Ok(quality_level)
}

// Elapsed time: 8912006 us
// Memory Used: 427013.2 kb
pub fn get_max_geodes() -> Result<isize> {
    let blueprints = Blueprint::from(get_input(INPUT)?);
    let mut geodes = 1;
    for i in 0..3 {
        let blueprint = blueprints.get(i).unwrap();
        geodes *= blueprint.simulate_blueprint(32);
    }
    Ok(geodes)
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
    Unknown = 4,
}

impl Resource {
    fn new(resource: &str) -> Self {
        match resource {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            _ => Resource::Unknown,
        }
    }
}

type Recipe = [isize; 4];

#[derive(Hash, Eq, PartialEq, Clone)]
struct Blueprint {
    number: isize,
    costs: [Recipe; 4],
}

impl Blueprint {
    fn new(number: isize, costs: [Recipe; 4]) -> Self {
        Blueprint { number, costs }
    }

    fn from(input: String) -> Vec<Self> {
        let re = Regex::new(r"\d+|ore|clay|obsidian|geode|\.").unwrap();
        input
            .lines()
            .into_iter()
            .map(|line| {
                let mut info = re.find_iter(line);
                let number = info.next().unwrap().as_str().parse().unwrap();
                let mut costs = [Recipe::default(); 4];
                while let Some(resource) = info.next() {
                    let mut cost = [0; 4];
                    loop {
                        let resource_num = match info.next().unwrap().as_str().parse() {
                            Ok(val) => val,
                            Err(_) => break,
                        };
                        let next_resource = info.next().unwrap().as_str();
                        cost[Resource::new(next_resource) as usize] = resource_num;
                    }
                    costs[Resource::new(resource.as_str()) as usize] = cost;
                }
                Blueprint::new(number, costs)
            })
            .collect()
    }

    fn get_max_robots(&self) -> [isize; 4] {
        let mut max_robots = [0, 0, 0, isize::MAX];
        for cost in self.costs {
            for i in 0..3 {
                max_robots[i] = max(max_robots[i], cost[i]);
            }
        }
        max_robots
    }

    fn simulate_blueprint(&self, time: usize) -> isize {
        let start = State::start(self.clone(), time);
        let max_robots = self.get_max_robots();
        let mut states = Vec::from([start.clone()]);
        let mut visited = HashSet::from([start]);
        let mut geode_max = 0;
        while !states.is_empty() {
            states = states
                .iter()
                .map(|state| {
                    let mut new_states = Vec::new();
                    if state.time == 0 {
                        geode_max = max(geode_max, state.resources[Resource::Geode as usize])
                    } else {
                        new_states.append(&mut state.buy_robots(&max_robots));
                        for new_state in &mut new_states {
                            for resource in RESOURCE_LIST {
                                let robot_count = state.robots[resource.clone() as usize];
                                new_state.resources[resource as usize] += robot_count;
                            }
                            new_state.time -= 1;
                        }
                    }
                    new_states = new_states
                        .iter()
                        .filter_map(|new_state| {
                            if visited.insert(new_state.clone()) {
                                Some(new_state.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    new_states
                })
                .flatten()
                .collect();
        }
        geode_max
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    blueprint: Blueprint,
    robots: [isize; 4],
    resources: [isize; 4],
    time: usize,
}

impl State {
    fn new(blueprint: Blueprint, robots: [isize; 4], resources: [isize; 4], time: usize) -> Self {
        State {
            blueprint,
            robots,
            resources,
            time,
        }
    }

    fn start(blueprint: Blueprint, time: usize) -> Self {
        let robots = [1, 0, 0, 0];
        let resources = [0; 4];
        State::new(blueprint, robots, resources, time)
    }

    fn buy_robots(&self, max_robots: &[isize; 4]) -> Vec<Self> {
        let mut new_states = Vec::with_capacity(5);
        'OUTER: for resource in RESOURCE_LIST {
            let ind = resource.clone() as usize;
            let robot_num = self.robots[ind];
            if max_robots[ind] < robot_num + 1 {
                continue;
            }
            let mut buyable = false;
            let costs = self.blueprint.costs[ind];
            for i in 0..costs.len() {
                let cost = costs[i];
                let available_resource = self.resources[i];
                if available_resource < cost {
                    continue 'OUTER;
                }
                let resource_robot = self.robots[i];
                buyable = buyable || available_resource - resource_robot < cost;
            }
            if !buyable {
                continue;
            }
            let mut new_state = self.clone();
            new_state.robots[ind] += 1;
            for i in 0..costs.len() {
                let cost = costs[i];
                new_state.resources[i] -= cost;
            }
            if resource == Resource::Geode {
                return Vec::from([new_state]);
            }
            new_states.push(new_state);
        }
        new_states.push(self.clone());
        new_states
    }
}
