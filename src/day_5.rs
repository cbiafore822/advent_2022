use crate::get_input;
use regex::Regex;
use std::{io::Result, str::Lines};

const INPUT: &str = "inputs/day_5.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 3773 us
// Memory Used: 93.231445 kb
pub fn get_tops() -> Result<String> {
    let input = get_input(INPUT)?;
    let mut lines = input.lines();
    let mut stacks = get_stacks(&mut lines);
    let instructions: Vec<&str> = lines.collect();
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<usize> = re
        .find_iter(&instructions.concat())
        .filter_map(|d| d.as_str().parse().ok())
        .collect();
    let mut nums = nums.into_iter();
    while let (Some(amount), Some(start), Some(end)) = (nums.next(), nums.next(), nums.next()) {
        for _i in 0..amount {
            let c = stacks[start - 1].pop().unwrap();
            stacks[end - 1].push(c);
        }
    }
    let tops: Vec<String> = stacks
        .into_iter()
        .map(|mut v| v.pop().unwrap().to_string())
        .collect();
    Ok(tops.concat())
}

// Elapsed time: 3826 us
// Memory Used: 93.231445 kb
pub fn get_tops_2() -> Result<String> {
    let input = get_input(INPUT)?;
    let mut lines = input.lines();
    let mut stacks = get_stacks(&mut lines);
    let instructions: Vec<&str> = lines.collect();
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<usize> = re
        .find_iter(&instructions.concat())
        .filter_map(|d| d.as_str().parse().ok())
        .collect();
    let mut temp = Vec::new();
    let mut nums = nums.into_iter();
    while let (Some(amount), Some(start), Some(end)) = (nums.next(), nums.next(), nums.next()) {
        for _i in 0..amount {
            let c = stacks[start - 1].pop().unwrap();
            temp.push(c);
        }
        for _i in 0..amount {
            stacks[end - 1].push(temp.pop().unwrap())
        }
    }
    let tops: Vec<String> = stacks
        .into_iter()
        .map(|mut v| v.pop().unwrap().to_string())
        .collect();
    Ok(tops.concat())
}

fn get_stacks(lines: &mut Lines) -> Vec<Vec<char>> {
    let mut line = lines.next().unwrap();
    let mut res: Vec<Vec<char>> = vec![Vec::new(); (line.len() + 1) / 4];
    while line.contains('[') {
        let letters = line.as_bytes();
        for i in 0..res.len() {
            if letters[i * 4 + 1] != (' ' as u8) {
                res[i].push(letters[i * 4 + 1] as char);
            }
        }
        line = lines.next().unwrap();
    }
    for i in 0..res.len() {
        res[i].reverse();
    }
    res
}
