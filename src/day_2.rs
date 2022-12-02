use std::{
    fs::File,
    io::{Read, Result},
};

const INPUT: &str = "inputs/day_2.txt";
const TEST: &str = "inputs/test.txt";

pub fn rps_score() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut res = 0;
    for game in input.split("\n") {
        res += match game.trim() {
            "A Y" => 8,
            "B Z" => 9,
            "C X" => 7,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "A Z" => 3,
            "B X" => 1,
            "C Y" => 2,
            _ => 0,
        };
    }
    Ok(res)
}

pub fn rps_correct_score() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut res = 0;
    for game in input.split("\n") {
        res += match game.trim() {
            "A Z" => 8,
            "B Z" => 9,
            "C Z" => 7,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            _ => 0,
        };
    }
    Ok(res)
}

fn get_input(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
