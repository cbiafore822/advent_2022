use crate::get_input;
use std::{cmp::max, collections::BinaryHeap, io::Result};

const INPUT: &str = "inputs/day_1.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 762 us
// Memory Used: 10.375 kb
pub fn max_calories() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut res = 0;
    let mut curr = 0;
    for calorie_str in input.lines() {
        match calorie_str.parse::<isize>() {
            Ok(calorie) => curr += calorie,
            Err(_) => {
                res = max(res, curr);
                curr = 0
            }
        }
    }
    Ok(res)
}

// Elapsed time: 812 us
// Memory Used: 16.375 kb
pub fn max3_calories() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut heap = BinaryHeap::new();
    let mut curr = 0;
    for calorie_str in input.lines() {
        match calorie_str.parse::<isize>() {
            Ok(calorie) => curr += calorie,
            Err(_) => {
                heap.push(curr);
                curr = 0
            }
        }
    }
    Ok(heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap())
}
