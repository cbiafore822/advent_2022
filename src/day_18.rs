use crate::get_input;
use std::{cmp::max, collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_18.txt";
const TEST: &str = "inputs/test.txt";

const NEIGHBORS: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

// Elapsed time: 6354 us
// Memory Used: 164.95117 kb
pub fn get_surface_area() -> Result<usize> {
    let input = get_input(INPUT)?;
    let coords: HashSet<Coord> = input.lines().map(|line| Coord::from(line)).collect();
    let mut sa = 0;
    for coord in &coords {
        sa += 6;
        for neighbor in NEIGHBORS {
            if coords.contains(&coord.move_by(neighbor)) {
                sa -= 1
            }
        }
    }
    Ok(sa)
}

// Elapsed time: 4170398 us
// Memory Used: 189.9668 kb
pub fn get_surface_area_2() -> Result<usize> {
    let input = get_input(INPUT)?;
    let coords: HashSet<Coord> = input.lines().map(|line| Coord::from(line)).collect();
    let mut max_pos = [0; 3];
    let mut sa = 0;
    for coord in &coords {
        max_pos[0] = max(max_pos[0], coord.x);
        max_pos[1] = max(max_pos[1], coord.y);
        max_pos[2] = max(max_pos[2], coord.z);
    }
    for coord in &coords {
        sa += 6;
        for neighbor in NEIGHBORS {
            let surface_point = coord.move_by(neighbor);
            let mut visited = HashSet::from([surface_point.clone()]);
            if coords.contains(&surface_point)
                || !surface_point.reach_edge(&coords, &mut visited, &max_pos)
            {
                sa -= 1
            }
        }
    }
    Ok(sa)
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Coord { x, y, z }
    }

    fn from(input: &str) -> Self {
        let nums: Vec<isize> = input.split(",").map(|num| num.parse().unwrap()).collect();
        Coord {
            x: *nums.get(0).unwrap(),
            y: *nums.get(1).unwrap(),
            z: *nums.get(2).unwrap(),
        }
    }

    fn move_by(&self, vector: (isize, isize, isize)) -> Self {
        Coord::new(self.x - vector.0, self.y - vector.1, self.z - vector.2)
    }

    fn reach_edge(
        &self,
        coords: &HashSet<Coord>,
        visited: &mut HashSet<Coord>,
        max_pos: &[isize; 3],
    ) -> bool {
        if self.x <= 0
            || max_pos[0] <= self.x
            || self.y <= 0
            || max_pos[1] <= self.y
            || self.z <= 0
            || max_pos[2] <= self.z
        {
            return true;
        }
        let mut res = false;
        for neighbor in NEIGHBORS {
            let next = self.move_by(neighbor);
            if !coords.contains(&next) && visited.insert(next.clone()) {
                res = next.reach_edge(coords, visited, max_pos);
            }
            if res {
                return res;
            }
        }
        res
    }
}
