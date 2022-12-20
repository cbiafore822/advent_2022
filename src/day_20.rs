use crate::get_input;
use std::io::Result;

const INPUT: &str = "inputs/day_20.txt";
const TEST: &str = "inputs/test.txt";
const KEY: isize = 811589153;

// Elapsed time: 318577 us
// Memory Used: 218.38965 kb
pub fn get_grove_coordinates() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut vals: Vec<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|val| (val.0, val.1.parse().unwrap()))
        .collect();
    mix(&mut vals);
    let (i, _) = vals
        .iter()
        .enumerate()
        .find(|(_, (_, val))| *val == 0)
        .unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|ind| vals[(i + ind) % vals.len()].1)
        .sum())
}

// Elapsed time: 3545998 us
// Memory Used: 218.38965 kb
pub fn get_grove_coordinates_with_key() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut vals: Vec<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|val| (val.0, val.1.parse::<isize>().unwrap() * KEY))
        .collect();
    for _i in 0..10 {
        mix(&mut vals);
    }
    let (i, _) = vals
        .iter()
        .enumerate()
        .find(|(_, (_, val))| *val == 0)
        .unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|ind| vals[(i + ind) % vals.len()].1)
        .sum())
}

fn mix(vals: &mut [(usize, isize)]) {
    for i in 0..vals.len() {
        let (pos, &val) = vals
            .iter()
            .enumerate()
            .find(|(_, (val, _))| *val == i)
            .unwrap();
        let j = (pos as isize + val.1).rem_euclid(vals.len() as isize - 1) as usize;
        if pos < j {
            vals.copy_within((pos + 1)..=j, pos)
        } else if pos > j {
            vals.copy_within(j..pos, j + 1);
        }
        vals[j] = val;
    }
}
