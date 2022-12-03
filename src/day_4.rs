use std::{
    fs::File,
    io::{Read, Result},
};

const INPUT: &str = "inputs/day_4.txt";
const TEST: &str = "inputs/test.txt";

fn get_input(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
