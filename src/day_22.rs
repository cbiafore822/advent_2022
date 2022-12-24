use crate::get_input;
use regex::Regex;
use std::io::Result;

const INPUT: &str = "inputs/day_22.txt";
const TEST: &str = "inputs/test.txt";

pub fn get_password() -> Result<usize> {
    let input = get_input(INPUT)?;
    let (grid, instruction_str) = build_grid(input);
    let (mut curr, mut direction) = ((0, grid[0].iter().position(|c| *c == '.').unwrap()), 0);
    let re = Regex::new(r"\d+|R|L").unwrap();
    let mut instructions = re.find_iter(&instruction_str);
    while let Some(instruction) = instructions.next() {
        (curr, direction) = move_pos(
            &grid,
            &mut curr,
            &mut direction,
            instruction.as_str(),
            false,
        );
    }
    Ok((curr.0 + 1) * 1000 + (curr.1 + 1) * 4 + direction)
}

pub fn get_cube_password() -> Result<usize> {
    let input = get_input(INPUT)?;
    let (grid, instruction_str) = build_grid(input);
    let (mut curr, mut direction) = ((0, grid[0].iter().position(|c| *c == '.').unwrap()), 0);
    let re = Regex::new(r"\d+|R|L").unwrap();
    let mut instructions = re.find_iter(&instruction_str);
    while let Some(instruction) = instructions.next() {
        (curr, direction) = move_pos(&grid, &mut curr, &mut direction, instruction.as_str(), true);
    }
    Ok((curr.0 + 1) * 1000 + (curr.1 + 1) * 4 + direction)
}

fn move_pos(
    grid: &Vec<Vec<char>>,
    curr: &mut (usize, usize),
    direction: &mut usize,
    instruction: &str,
    is_cube: bool,
) -> ((usize, usize), usize) {
    match instruction {
        "R" => (*curr, (*direction + 1).rem_euclid(4)),
        "L" => (
            *curr,
            (*direction as isize - 1).rem_euclid(4 as isize) as usize,
        ),
        _ => {
            let dist = instruction.parse().unwrap();
            for _i in 0..dist {
                let (ncurr, ndirection) = match direction {
                    0 | 2 => get_next(grid, *curr, *direction, false, is_cube),
                    1 | 3 => get_next(grid, *curr, *direction, true, is_cube),
                    _ => panic!("Bad Instruction!"),
                };
                if ncurr == *curr && ndirection == *direction {
                    break;
                }
                (*curr, *direction) = (ncurr, ndirection);
            }
            (*curr, *direction)
        }
    }
}

fn get_next(
    grid: &Vec<Vec<char>>,
    mut curr: (usize, usize),
    mut direction: usize,
    is_vertical: bool,
    is_cube: bool,
) -> ((usize, usize), usize) {
    let (mut ncurr, mut ndirection) = (curr, direction);
    let go = if direction == 0 || direction == 1 {
        1
    } else {
        -1
    };
    if !is_cube && !is_vertical {
        let row = grid.get(curr.0).unwrap();
        ncurr.1 = wrap1(row, ncurr.1, go);
    } else if !is_cube && is_vertical {
        let col: &Vec<char> = &grid
            .iter()
            .map(|chars| *chars.get(curr.1).unwrap())
            .collect();
        ncurr.0 = wrap1(col, ncurr.0, go);
    } else {
        (ncurr, ndirection) = wrap2(grid, ncurr, ndirection, go, is_vertical);
    }
    match grid.get(ncurr.0).unwrap().get(ncurr.1).unwrap() {
        '#' => (),
        '.' => (curr, direction) = (ncurr, ndirection),
        _ => panic!("Bad location"),
    }
    (curr, direction)
}

fn wrap1(rc: &Vec<char>, mut new_pos: usize, go: isize) -> usize {
    new_pos = (new_pos as isize + go).rem_euclid(rc.len() as isize) as usize;
    while *rc.get(new_pos).unwrap() == ' ' {
        new_pos = (new_pos as isize + go).rem_euclid(rc.len() as isize) as usize;
    }
    new_pos
}

fn wrap2(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    direction: usize,
    go: isize,
    is_vertical: bool,
) -> ((usize, usize), usize) {
    let mut new_pos;
    let mut new_direction = direction;
    if is_vertical {
        new_pos = (
            (pos.0 as isize + go).rem_euclid(grid.len() as isize) as usize,
            pos.1,
        );
        if new_pos.0 as isize == pos.0 as isize + go
            && *grid.get(new_pos.0).unwrap().get(new_pos.1).unwrap() != ' '
        {
            return (new_pos, new_direction);
        }
    } else {
        new_pos = (
            pos.0,
            (pos.1 as isize + go).rem_euclid(grid.get(0).unwrap().len() as isize) as usize,
        );
        if new_pos.1 as isize == pos.1 as isize + go
            && *grid.get(new_pos.0).unwrap().get(new_pos.1).unwrap() != ' '
        {
            return (new_pos, new_direction);
        }
    }
    (new_pos, new_direction) = match ((pos.0 / 50, pos.1 / 50), direction) {
        ((0, 1), 3) => ((3, 0), 0),
        ((0, 1), 2) => ((2, 0), 0),
        ((0, 2), 3) => ((3, 0), 3),
        ((0, 2), 0) => ((2, 1), 2),
        ((0, 2), 1) => ((1, 1), 2),
        ((1, 1), 0) => ((0, 2), 3),
        ((1, 1), 2) => ((2, 0), 1),
        ((2, 0), 3) => ((1, 1), 0),
        ((2, 0), 2) => ((0, 1), 0),
        ((2, 1), 0) => ((0, 2), 2),
        ((2, 1), 1) => ((3, 0), 2),
        ((3, 0), 0) => ((2, 1), 3),
        ((3, 0), 1) => ((0, 2), 1),
        ((3, 0), 2) => ((0, 1), 1),
        _ => panic!("Bad wrap"),
    };
    let (row_idx, col_idx) = (pos.0 % 50, pos.1 % 50);
    let i = match direction {
        2 => 49 - row_idx,
        0 => row_idx,
        3 => col_idx,
        1 => 49 - col_idx,
        _ => panic!("Bad Direction"),
    };
    new_pos = match new_direction {
        2 => (new_pos.0 * 50 + 49 - i, new_pos.1 * 50 + 49),
        0 => (new_pos.0 * 50 + i, new_pos.1 * 50),
        3 => (new_pos.0 * 50 + 49, new_pos.1 * 50 + i),
        1 => (new_pos.0 * 50, new_pos.1 * 50 + 49 - i),
        _ => panic!("Bad Direction"),
    };
    (new_pos, new_direction)
}

fn build_grid(input: String) -> (Vec<Vec<char>>, String) {
    let size = input
        .lines()
        .map(|line| match line.chars().next() {
            Some(val) => {
                if val.is_alphanumeric() {
                    return 0;
                }
                line.len()
            }
            None => return 0,
        })
        .max()
        .unwrap();
    let grid = input
        .lines()
        .filter_map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            match chars.get(0) {
                Some(val) => {
                    if val.is_alphanumeric() {
                        return None;
                    }
                    while chars.len() < size {
                        chars.push(' ');
                    }
                    Some(chars)
                }
                None => None,
            }
        })
        .collect();
    let instructions = input.lines().last().unwrap().to_string();
    (grid, instructions)
}
