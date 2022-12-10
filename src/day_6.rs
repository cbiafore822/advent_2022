use crate::get_input;
use std::{collections::HashMap, io::Result};

const INPUT: &str = "inputs/day_6.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 2269 us
// Memory Used: 28.136719 kb
pub fn get_start_of_packet() -> Result<isize> {
    let stream: Vec<char> = get_input(INPUT)?.chars().collect();
    let mut seen: HashMap<char, isize> = HashMap::new();
    for i in 0..stream.len() {
        *seen.entry(stream[i]).or_insert(0) += 1;
        if i < 4 {
            continue;
        }
        let k = stream[i - 4];
        let val = seen.get(&k).unwrap() - 1;
        if val != 0 {
            seen.insert(k, val);
        } else {
            seen.remove(&k);
        }
        if seen.len() == 4 {
            return Ok((i + 1).try_into().unwrap());
        }
    }
    Ok(-1)
}

// Elapsed time: 3474 us
// Memory Used: 28.136719 kb
pub fn get_start_of_message() -> Result<isize> {
    let stream: Vec<char> = get_input(INPUT)?.chars().collect();
    let mut seen: HashMap<char, isize> = HashMap::new();
    for i in 0..stream.len() {
        *seen.entry(stream[i]).or_insert(0) += 1;
        if i < 14 {
            continue;
        }
        let k = stream[i - 14];
        let val = seen.get(&k).unwrap() - 1;
        if val != 0 {
            seen.insert(k, val);
        } else {
            seen.remove(&k);
        }
        if seen.len() == 14 {
            return Ok((i + 1).try_into().unwrap());
        }
    }
    Ok(-1)
}
