use crate::get_input;
use std::{
    cmp::{max, min},
    collections::HashSet,
    io::Result,
};

const INPUT: &str = "inputs/day_14.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 149799 us
// Memory Used: 70.16113 kb
pub fn find_resting_sand_with_void() -> Result<usize> {
    let mut simulator = SandSimulator::new(get_input(INPUT)?, true);
    simulator.simulate_sand();
    Ok(simulator.resting.len())
}

// Elapsed time: 3527482 us
// Memory Used: 837.16113 kb
pub fn find_resting_sand() -> Result<usize> {
    let mut simulator = SandSimulator::new(get_input(INPUT)?, false);
    simulator.simulate_sand();
    Ok(simulator.resting.len())
}

struct SandSimulator {
    rocks: HashSet<(isize, isize)>,
    resting: HashSet<(isize, isize)>,
    falling: Vec<(isize, isize)>,
    bottom: isize,
    with_void: bool,
}

impl SandSimulator {
    fn new(input: String, with_void: bool) -> Self {
        let mut bottom = 0;
        let mut rocks = HashSet::new();
        for path in input.lines() {
            let coords_str = path.split(" -> ");
            let coords: Vec<(isize, isize)> = coords_str
                .map(|coord| {
                    let (x, y) = coord.split_once(",").unwrap();
                    (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
                })
                .collect();
            bottom = max(bottom, coords[0].1);
            for i in 1..coords.len() {
                let (px, py) = coords[i - 1];
                let (cx, cy) = coords[i];
                let (dx, dy) = (cx - px, cy - py);
                let (sx, sy) = (min(px, cx), min(py, cy));
                for j in 0..=dx.abs() {
                    rocks.insert((sx + j, sy));
                }
                for j in 0..=dy.abs() {
                    rocks.insert((sx, sy + j));
                }
                bottom = max(bottom, cy);
            }
        }
        if !with_void {
            bottom += 2;
        }
        SandSimulator {
            rocks,
            resting: HashSet::new(),
            falling: Vec::new(),
            bottom,
            with_void,
        }
    }

    fn simulate_sand(&mut self) {
        let mut stop = false;
        while !stop {
            self.falling.push((500, 0));
            self.falling = self
                .falling
                .iter()
                .filter_map(|sand| {
                    let (x, y) = *sand;
                    if (self.with_void && y >= self.bottom)
                        || (!self.with_void && self.resting.contains(&(500, 0)))
                    {
                        stop = true;
                        None
                    } else if !self.rocks.contains(&(x, y + 1))
                        && !self.resting.contains(&(x, y + 1))
                        && (self.with_void || y + 1 < self.bottom)
                    {
                        Some((x, y + 1))
                    } else if !self.rocks.contains(&(x - 1, y + 1))
                        && !self.resting.contains(&(x - 1, y + 1))
                        && (self.with_void || y + 1 < self.bottom)
                    {
                        Some((x - 1, y + 1))
                    } else if !self.rocks.contains(&(x + 1, y + 1))
                        && !self.resting.contains(&(x + 1, y + 1))
                        && (self.with_void || y + 1 < self.bottom)
                    {
                        Some((x + 1, y + 1))
                    } else {
                        self.resting.insert((x, y));
                        None
                    }
                })
                .collect();
        }
    }
}
