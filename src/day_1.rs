use std::{
    cmp::max,
    collections::BinaryHeap,
    fs::File,
    io::{Read, Result},
};

const INPUT: &str = "inputs/day_1.txt";
const TEST: &str = "inputs/test.txt";

pub fn max_calories() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut res = 0;
    let mut curr = 0;
    for calorie_str in input.split("\n") {
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

pub fn max3_calories() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut heap = BinaryHeap::new();
    let mut curr = 0;
    for calorie_str in input.split("\n") {
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

fn get_input(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
