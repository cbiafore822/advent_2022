use crate::get_input;
use std::io::Result;

const INPUT: &str = "inputs/day_25.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 751 us
// Memory Used: 1.3779297 kb
pub fn get_snafu_number() -> Result<String> {
    let input = get_input(INPUT)?;
    let total_fuel = add_snafu_numbers(input);
    Ok(decimal_to_snafu(total_fuel))
}

fn add_snafu_numbers(numbers: String) -> isize {
    numbers
        .lines()
        .map(|number| {
            let mut base = 1;
            number
                .chars()
                .rev()
                .map(|c| {
                    let val = match c {
                        '2' => base * 2,
                        '1' => base,
                        '0' => 0,
                        '-' => base * -1,
                        '=' => base * -2,
                        _ => panic!("Bad Number"),
                    };
                    base *= 5;
                    val
                })
                .sum::<isize>()
        })
        .sum()
}

fn decimal_to_snafu(mut number: isize) -> String {
    let mut s_number = String::new();
    let mut base = 1;
    while number != 0 {
        let digit = (number / base) % 5;
        match digit {
            0 => s_number.push('0'),
            1 => {
                s_number.push('1');
                number -= base;
            }
            2 => {
                s_number.push('2');
                number -= 2 * base;
            }
            3 => {
                s_number.push('=');
                number += 2 * base;
            }
            4 => {
                s_number.push('-');
                number += base;
            }
            _ => panic!("Bad Number")
        }
        base *= 5;
    }
    s_number.chars().rev().collect()
}
