use crate::get_input;
use std::{collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_12.txt";
const TEST: &str = "inputs/test.txt";

const NEIGHBORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// Elapsed time: 13771 us
// Memory Used: 212.96191 kb
pub fn find_minimum_path() -> Result<usize> {
    let (map, start, end) = get_map_info(get_input(INPUT)?);
    bfs(map, start, end, true)
}

// Elapsed time: 10604 us
// Memory Used: 213.21191 kb
pub fn find_best_start() -> Result<usize> {
    let (map, end, start) = get_map_info(get_input(INPUT)?);
    bfs(map, start, end, false)
}

fn get_map_info(input: String) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let map: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = Some((i, j));
                        return 'a' as u8;
                    } else if c == 'E' {
                        end = Some((i, j));
                        return 'z' as u8;
                    }
                    c as u8
                })
                .collect()
        })
        .collect();
    (map, start.unwrap(), end.unwrap())
}

fn bfs(
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    go_up: bool,
) -> Result<usize> {
    let (m, n) = (map.len(), map[0].len());
    let mut queue = Vec::from([start]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut res = 0;
    while !queue.is_empty() {
        let mut temp = Vec::new();
        for curr in queue {
            let curr_level = map[curr.0][curr.1];
            for neighbor in NEIGHBORS {
                let dx = neighbor.0.checked_add(curr.0.try_into().unwrap());
                let dy = neighbor.1.checked_add(curr.1.try_into().unwrap());
                match (dx, dy) {
                    (None, _) | (_, None) => continue,
                    (Some(nx), Some(ny)) => {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if nx >= m || ny >= n || visited.contains(&(nx, ny)) {
                            continue;
                        }
                        let new_level = map[nx][ny];
                        if (go_up && curr_level + 1 < new_level)
                            || (!go_up && curr_level > new_level + 1)
                        {
                            continue;
                        }
                        if (go_up && (nx, ny) == end) || (!go_up && new_level == ('a' as u8)) {
                            return Ok(res + 1);
                        }
                        visited.insert((nx, ny));
                        temp.push((nx, ny));
                    }
                }
            }
        }
        res += 1;
        queue = temp
    }
    panic!("Path Not Found")
}
