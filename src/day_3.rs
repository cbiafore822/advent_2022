use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Result},
};

const INPUT: &str = "inputs/day_3.txt";
const TEST: &str = "inputs/test.txt";

pub fn get_rucksack_sum() -> Result<isize> {
    let rucksacks = get_input(INPUT)?;
    let mut res = 0;
    let mut similar = Vec::new();
    for rucksack in rucksacks.lines() {
        let mid = rucksack.len() / 2;
        let ruck_1: HashSet<char> = rucksack[0..mid].chars().collect();
        let ruck_2: HashSet<char> = rucksack[mid..rucksack.len()].chars().collect();
        let same = ruck_1.intersection(&ruck_2).next().unwrap().to_owned();
        similar.push(same);
    }
    for letter in similar {
        res += match letter {
            'a'..='z' => letter as isize - 96,
            'A'..='Z' => letter as isize - 38,
            _ => 0,
        }
    }
    Ok(res)
}

pub fn get_group_sum() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut rucksacks = input.lines();
    let mut res = 0;
    let mut similar = Vec::new();
    while let Some(ruck_1) = rucksacks.next() {
        let ruck_1: HashSet<char> = ruck_1.chars().collect();
        let ruck_2: HashSet<char> = rucksacks.next().unwrap().chars().collect();
        let ruck_3: HashSet<char> = rucksacks.next().unwrap().chars().collect();
        let same: HashSet<char> = ruck_1.intersection(&ruck_2).copied().collect();
        let same = same.intersection(&ruck_3).next().unwrap().to_owned();
        similar.push(same);
    }
    for letter in similar {
        res += match letter {
            'a'..='z' => letter as isize - 96,
            'A'..='Z' => letter as isize - 38,
            _ => 0,
        }
    }
    Ok(res)
}

fn get_input(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
