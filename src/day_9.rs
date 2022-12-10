use crate::get_input;
use std::{collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_9.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 8559 us
// Memory Used: 212.35352 kb
pub fn find_rope_locations() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut rope = Rope::new(2);
    for command in input.lines() {
        let (direction, val) = command.split_once(" ").unwrap();
        for _i in 0..val.parse::<usize>().unwrap() {
            visited.insert(rope.move_rope(direction));
        }
    }
    Ok(visited.len())
}

// Elapsed time: 15980 us
// Memory Used: 110.478516 kb
pub fn find_long_rope_locations() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut rope = Rope::new(10);
    for command in input.lines() {
        let (direction, val) = command.split_once(" ").unwrap();
        for _i in 0..val.parse::<usize>().unwrap() {
            visited.insert(rope.move_rope(direction));
        }
    }
    Ok(visited.len())
}

struct Rope {
    knots: Vec<(isize, isize)>,
    length: usize,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            knots: vec![(0, 0); length],
            length: length,
        }
    }

    fn move_rope(&mut self, direction: &str) -> (isize, isize) {
        match direction {
            "R" => self.knots[0].0 += 1,
            "L" => self.knots[0].0 -= 1,
            "U" => self.knots[0].1 += 1,
            "D" => self.knots[0].1 -= 1,
            _ => (),
        }
        for i in 1..self.length {
            let dx = self.knots[i - 1].0 - self.knots[i].0;
            let dy = self.knots[i - 1].1 - self.knots[i].1;
            if dx == 0 && dy == 0 {
                break;
            } else if (dx == 0 || dy == 0) && (dx.abs() + dy.abs()) >= 2 {
                self.knots[i].0 += dx / 2;
                self.knots[i].1 += dy / 2;
            } else if (dx.abs() + dy.abs()) >= 3 {
                self.knots[i].0 += if dx.abs() == 1 { dx } else { dx / 2 };
                self.knots[i].1 += if dy.abs() == 1 { dy } else { dy / 2 };
            }
        }
        self.knots[self.length - 1]
    }
}
