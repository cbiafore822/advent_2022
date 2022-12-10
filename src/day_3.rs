use crate::get_input;
use std::{collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_3.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 5296 us
// Memory Used: 10.2109375 kb
pub fn get_rucksack_sum() -> Result<isize> {
    let rucksacks = get_input(INPUT)?;
    let mut res = 0;
    for rucksack in rucksacks.lines() {
        let mid = rucksack.len() / 2;
        let ruck_1: HashSet<char> = rucksack[..mid].chars().collect();
        let ruck_2: HashSet<char> = rucksack[mid..].chars().collect();
        let letter = ruck_1.intersection(&ruck_2).next().unwrap().to_owned();
        res += match letter {
            'a'..='z' => letter as isize - 96,
            'A'..='Z' => letter as isize - 38,
            _ => 0,
        }
    }
    Ok(res)
}

// Elapsed time: 5362 us
// Memory Used: 10.6328125 kb
pub fn get_group_sum() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut rucksacks = input.lines();
    let mut res = 0;
    while let Some(ruck_1) = rucksacks.next() {
        let ruck_1: HashSet<char> = ruck_1.chars().collect();
        let ruck_2: HashSet<char> = rucksacks.next().unwrap().chars().collect();
        let ruck_3: HashSet<char> = rucksacks.next().unwrap().chars().collect();
        let same: HashSet<char> = ruck_1.intersection(&ruck_2).copied().collect();
        let letter = same.intersection(&ruck_3).next().unwrap().to_owned();
        res += match letter {
            'a'..='z' => letter as isize - 96,
            'A'..='Z' => letter as isize - 38,
            _ => 0,
        }
    }
    Ok(res)
}
