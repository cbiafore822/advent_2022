use crate::get_input;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_24.txt";
const TEST: &str = "inputs/test.txt";
const NEIGHBORS: [(isize, isize); 5] = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

// Elapsed time: 182536932 us
// Memory Used: 9802.286 kb
pub fn find_shortest_path() -> Result<usize> {
    let input = get_input(INPUT)?;
    let valley = Valley::from(input);
    Ok(valley.find_shortest_path(valley.start, valley.end, 0))
}

// Elapsed time: 430248445 us
// Memory Used: 9824.692 kb
pub fn find_shortest_snack_path() -> Result<usize> {
    let input = get_input(INPUT)?;
    let valley = Valley::from(input);
    let p1 = valley.find_shortest_path(valley.start, valley.end, 0);
    let p2 = valley.find_shortest_path(valley.end, valley.start, p1);
    Ok(valley.find_shortest_path(valley.start, valley.end, p2))
}

#[derive(Debug)]
struct Valley {
    start: (isize, isize),
    end: (isize, isize),
    size: (isize, isize),
    walls: HashSet<(isize, isize)>,
    blizzards: HashMap<(isize, isize), char>,
}

impl Valley {
    fn new(
        start: (isize, isize),
        end: (isize, isize),
        size: (isize, isize),
        walls: HashSet<(isize, isize)>,
        blizzards: HashMap<(isize, isize), char>,
    ) -> Self {
        Valley {
            start,
            end,
            size,
            walls,
            blizzards,
        }
    }

    fn from(input: String) -> Self {
        let start = (0, input.find('.').unwrap() as isize);
        let end = (
            input.lines().count() as isize - 1,
            input.lines().last().unwrap().find('.').unwrap() as isize,
        );
        let size = (
            input.lines().count() as isize,
            input.lines().next().unwrap().len() as isize,
        );
        let walls = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(j, c)| match c {
                        '#' => Some((i as isize, j as isize)),
                        _ => None,
                    })
                    .collect::<HashSet<(isize, isize)>>()
            })
            .collect();
        let blizzards = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(j, c)| match c {
                        '>' | 'v' | '<' | '^' => Some(((i as isize, j as isize), c)),
                        _ => None,
                    })
                    .collect::<HashMap<(isize, isize), char>>()
            })
            .collect();
        Self::new(start, end, size, walls, blizzards)
    }

    fn get_blizzards_at_time(&self, time: usize) -> HashSet<(isize, isize)> {
        self.blizzards
            .iter()
            .map(|(&pos, &c)| match c {
                '>' => (
                    pos.0,
                    (pos.1 + time as isize - 1).rem_euclid(self.size.1 - 2) + 1,
                ),
                '<' => (
                    pos.0,
                    (pos.1 - time as isize - 1).rem_euclid(self.size.1 - 2) + 1,
                ),
                'v' => (
                    (pos.0 + time as isize - 1).rem_euclid(self.size.0 - 2) + 1,
                    pos.1,
                ),
                '^' => (
                    (pos.0 - time as isize - 1).rem_euclid(self.size.0 - 2) + 1,
                    pos.1,
                ),
                _ => panic!("Bad Blizzard Direction"),
            })
            .collect()
    }

    fn find_shortest_path(&self, start: (isize, isize), end: (isize, isize), start_time: usize) -> usize {
        let mut queue = Vec::from([(start, start_time)]);
        let mut visited = HashSet::from([(start, start_time)]);
        let mut time = usize::MAX;
        while !queue.is_empty() {
            queue = queue
                .iter()
                .flat_map(|(curr, t)| {
                    let mut next_states = Vec::new();
                    if *curr == end {
                        time = min(time, *t);
                    } else if ((end.0 - curr.0).abs() + (end.1 - curr.1).abs())
                        as usize
                        + *t
                        < time
                    {
                        let blizzards = self.get_blizzards_at_time(t + 1);
                        next_states = NEIGHBORS
                            .iter()
                            .filter_map(|neighbor| {
                                let next = ((curr.0 + neighbor.0, curr.1 + neighbor.1), t + 1);
                                if !(0..self.size.0).contains(&next.0 .0)
                                    || !(0..self.size.1).contains(&next.0 .1)
                                    || blizzards.contains(&next.0)
                                    || self.walls.contains(&next.0)
                                    || !visited.insert(next)
                                {
                                    None
                                } else {
                                    Some(next)
                                }
                            })
                            .collect()
                    }
                    next_states
                })
                .collect();
        }
        time
    }
}
