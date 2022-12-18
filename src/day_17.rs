use crate::get_input;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::Result,
};

const INPUT: &str = "inputs/day_17.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 108366 us
// Memory Used: 816.16504 kb
pub fn get_height() -> Result<usize> {
    let input = get_input(INPUT)?;
    Ok(find_height(input, 2022))
}

// Elapsed time: 145147 us
// Memory Used: 816.16504 kb
pub fn get_tall_height() -> Result<usize> {
    let input = get_input(INPUT)?;
    Ok(find_height(input, 1000000000000))
}

fn find_height(input: String, mut rocks: usize) -> usize {
    let mut jets = input.lines().next().unwrap().chars().enumerate().cycle();
    let mut shapes = ['-', '+', 'J', 'I', 'o'].iter().enumerate().cycle();
    let mut cache: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();
    let mut resting = HashSet::new();
    let mut height = 0;
    while rocks > 0 {
        let c = shapes.next().unwrap().clone();
        let mut shape = Shape::new(*c.1, height);
        loop {
            let jet = jets.next().unwrap();
            shape.move_shape(jet.1, &mut resting);
            if shape.move_shape('v', &mut resting) {
                for pos in &shape.positions {
                    height = max(height, pos.1);
                }
                rocks -= 1;
                if let Some((s_height, s_rocks)) = cache.insert(
                    (c.0, jet.0, column_heights(&resting, &height)),
                    (height, rocks),
                ) {
                    let cycle_len = s_rocks - rocks;
                    if rocks % cycle_len != 0 {
                        break;
                    }
                    let dh = height - s_height;
                    height += dh * (rocks / cycle_len);
                    rocks %= cycle_len;
                }
                break;
            };
        }
    }
    height
}

fn column_heights(resting: &HashSet<(usize, usize)>, height: &usize) -> [usize; 7] {
    let mut heights = [0; 7];
    for i in 1..=7 {
        for j in (0..=*height).rev() {
            if resting.contains(&(i, j)) || j == 0 {
                heights[i - 1] = height - j;
                break;
            }
        }
    }
    heights
}

struct Shape {
    positions: HashSet<(usize, usize)>,
}

impl Shape {
    fn new(shape: char, height: usize) -> Self {
        let mut positions = HashSet::new();
        match shape {
            '-' => {
                for i in 0..4 {
                    positions.insert((i + 3, height + 4));
                }
            }
            '+' => {
                for i in 0..3 {
                    positions.insert((i + 3, height + 5));
                    positions.insert((4, height + i + 4));
                }
            }
            'J' => {
                for i in 0..3 {
                    positions.insert((i + 3, height + 4));
                    positions.insert((5, height + i + 4));
                }
            }
            'I' => {
                for i in 0..4 {
                    positions.insert((3, height + 4 + i));
                }
            }
            'o' => {
                positions.insert((3, height + 5));
                positions.insert((3, height + 4));
                positions.insert((4, height + 5));
                positions.insert((4, height + 4));
            }
            _ => (),
        }
        Shape { positions }
    }

    fn move_shape(&mut self, gust: char, resting: &mut HashSet<(usize, usize)>) -> bool {
        let mut stop = false;
        let new_positions = match gust {
            '<' => self
                .positions
                .iter()
                .map(|position| {
                    let new_position = (position.0 - 1, position.1);
                    stop = stop
                        || !(1..=7).contains(&new_position.0)
                        || resting.contains(&new_position);
                    new_position
                })
                .collect(),
            '>' => self
                .positions
                .iter()
                .map(|position| {
                    let new_position = (position.0 + 1, position.1);
                    stop = stop
                        || !(1..=7).contains(&new_position.0)
                        || resting.contains(&new_position);
                    new_position
                })
                .collect(),
            'v' => self
                .positions
                .iter()
                .map(|position| {
                    let new_position = (position.0, position.1 - 1);
                    stop = stop || new_position.1 == 0 || resting.contains(&new_position);
                    new_position
                })
                .collect(),
            _ => HashSet::new(),
        };
        if !stop {
            self.positions = new_positions;
        } else if gust == 'v' {
            resting.extend(self.positions.iter());
        }
        stop
    }
}
