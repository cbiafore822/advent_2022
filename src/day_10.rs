use crate::get_input;
use std::io::Result;

const INPUT: &str = "inputs/day_10.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 586 us
// Memory Used: 1.1416016 kb
pub fn get_signal_strengths() -> Result<isize> {
    let instructions = get_input(INPUT)?;
    let mut signals = Vec::new();
    let mut clock = 0;
    let mut x = 1;
    for instruction in instructions.lines() {
        clock += 1;
        if (clock + 20) % 40 == 0 {
            signals.push(clock * x);
        }
        if instruction.starts_with("addx") {
            let (_op, val) = instruction.split_once(" ").unwrap();
            clock += 1;
            if (clock + 20) % 40 == 0 {
                signals.push(clock * x);
            }
            x += val.parse::<isize>().unwrap();
        }
    }
    Ok(signals[0..6].iter().sum())
}

// Elapsed time: 583 us
// Memory Used: 1.4228516 kb
pub fn draw_picture() -> Result<String> {
    let instructions = get_input(INPUT)?;
    let mut picture = String::from("\n");
    let mut clock = 0;
    let mut x = 1;
    for instruction in instructions.lines() {
        let sprite = (x - 1) % 40;
        picture.push(if sprite <= clock && clock <= sprite + 2 {
            '#'
        } else {
            '.'
        });
        clock += 1;
        if clock % 40 == 0 {
            picture.push('\n');
            clock = 0;
        }
        if instruction.starts_with("addx") {
            let (_op, val) = instruction.split_once(" ").unwrap();
            picture.push(if sprite <= clock && clock <= sprite + 2 {
                '#'
            } else {
                '.'
            });
            clock += 1;
            if clock % 40 == 0 {
                picture.push('\n');
                clock = 0;
            }
            x += val.parse::<isize>().unwrap();
        }
    }
    Ok(picture)
}
