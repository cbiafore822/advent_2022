use crate::get_input;
use std::io::Result;

const INPUT: &str = "inputs/day_4.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 992 us
// Memory Used: 11.272461 kb
pub fn count_full_overlaps() -> Result<isize> {
    let assignments = get_input(INPUT)?;
    let mut res = 0;
    for assignment in assignments.lines() {
        let (range_1, range_2) = assignment.split_once(",").unwrap();
        let elf_1_str = range_1.split_once("-").unwrap();
        let elf_1: (isize, isize) = (elf_1_str.0.parse().unwrap(), elf_1_str.1.parse().unwrap());
        let elf_2_str = range_2.split_once("-").unwrap();
        let elf_2: (isize, isize) = (elf_2_str.0.parse().unwrap(), elf_2_str.1.parse().unwrap());
        if (elf_1.0 <= elf_2.0 && elf_2.1 <= elf_1.1) || (elf_2.0 <= elf_1.0 && elf_1.1 <= elf_2.1)
        {
            res += 1;
        }
    }
    Ok(res)
}

// Elapsed time: 982 us
// Memory Used: 11.272461 kb
pub fn count_any_overlaps() -> Result<isize> {
    let assignments = get_input(INPUT)?;
    let mut res = 0;
    for assignment in assignments.lines() {
        let (range_1, range_2) = assignment.split_once(",").unwrap();
        let elf_1_str = range_1.split_once("-").unwrap();
        let elf_1: (isize, isize) = (elf_1_str.0.parse().unwrap(), elf_1_str.1.parse().unwrap());
        let elf_2_str = range_2.split_once("-").unwrap();
        let elf_2: (isize, isize) = (elf_2_str.0.parse().unwrap(), elf_2_str.1.parse().unwrap());
        if (elf_1.1 >= elf_2.0 && elf_1.0 <= elf_2.1) || (elf_2.1 >= elf_1.0 && elf_2.0 <= elf_1.1)
        {
            res += 1;
        }
    }
    Ok(res)
}
