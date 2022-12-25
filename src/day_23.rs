use crate::get_input;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_23.txt";
const TEST: &str = "inputs/test.txt";
const NEIGHBORS: [[(isize, isize); 3]; 4] = [
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, 1), (1, 0), (1, -1)],
    [(1, -1), (0, -1), (-1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
];

// Elapsed time: 147334 us
// Memory Used: 503.4043 kb
pub fn get_rectangle_area() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut elves: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '#' {
                        Some((i as isize, j as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>()
        })
        .flatten()
        .collect();
    let mut offset = 0;
    for _i in 0..10 {
        elves = move_elves(&elves, offset);
        offset = (offset + 1) % 4;
    }
    let (mut minx, mut maxx, mut miny, mut maxy) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for elf in &elves {
        minx = min(minx, elf.0);
        maxx = max(maxx, elf.0);
        miny = min(miny, elf.1);
        maxy = max(maxy, elf.1);
    }
    Ok((maxx - minx + 1) as usize * (maxy - miny + 1) as usize - elves.len())
}

// Elapsed time: 15461112 us
// Memory Used: 509.4043 kb
pub fn get_last_round() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut elves: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '#' {
                        Some((i as isize, j as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>()
        })
        .flatten()
        .collect();
    let mut count = 0;
    let mut offset = 0;
    loop {
        count += 1;
        let new_elves = move_elves(&elves, offset);
        offset = (offset + 1) % 4;
        if elves == new_elves {
            return Ok(count);
        }
        elves = new_elves;
    }
}

fn move_elves(elves: &HashSet<(isize, isize)>, offset: usize) -> HashSet<(isize, isize)> {
    let mut possible_pos: HashMap<(isize, isize), Vec<(isize, isize)>> =
        HashMap::with_capacity(elves.len());
    'OUTER: for elf in elves {
        if !NEIGHBORS.iter().any(|neighbor_arr| {
            neighbor_arr
                .iter()
                .any(|(dx, dy)| elves.contains(&(elf.0 + dx, elf.1 + dy)))
        }) {
            possible_pos.entry(*elf).or_default().push(*elf);
            continue;
        }
        for i in 0..4 {
            let neighbor_arr = NEIGHBORS[(offset + i).rem_euclid(4)];
            if neighbor_arr
                .iter()
                .all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy)))
            {
                possible_pos
                    .entry((elf.0 + neighbor_arr[1].0, elf.1 + neighbor_arr[1].1))
                    .or_default()
                    .push(*elf);
                continue 'OUTER;
            }
        }
        possible_pos.entry(*elf).or_default().push(*elf);
    }
    possible_pos
        .iter()
        .map(|(key, val)| {
            if val.len() == 1 {
                Vec::from([*key])
            } else {
                val.clone()
            }
        })
        .flatten()
        .collect()
}
