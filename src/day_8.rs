use crate::get_input;
use std::{cmp::max, collections::HashSet, io::Result, iter::Rev, ops::Range};

use itertools::Either;

const INPUT: &str = "inputs/day_8.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 5010 us
// Memory Used: 144.25 kb
pub fn get_visible_trees() -> Result<usize> {
    let input = get_input(INPUT)?;
    let heights: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();
    let mut counted: HashSet<(usize, usize)> = HashSet::new();
    let mut visible = 0;
    let (n, m) = (heights.len(), heights[0].len());
    visible += get_visible_from_direction(&heights, &mut counted, 0..n, Either::Left(0..m), false);
    visible += get_visible_from_direction(
        &heights,
        &mut counted,
        0..n,
        Either::Right((0..m).rev()),
        false,
    );
    visible += get_visible_from_direction(&heights, &mut counted, 0..n, Either::Left(0..m), true);
    visible += get_visible_from_direction(
        &heights,
        &mut counted,
        0..n,
        Either::Right((0..m).rev()),
        true,
    );
    Ok(visible)
}

fn get_visible_from_direction(
    heights: &Vec<Vec<isize>>,
    counted: &mut HashSet<(usize, usize)>,
    n: Range<usize>,
    m: Either<Range<usize>, Rev<Range<usize>>>,
    is_vertical: bool,
) -> usize {
    let mut visible = 0;
    let mut tallest = -1;
    for i in n {
        for j in m.clone() {
            let coord = if is_vertical { (j, i) } else { (i, j) };
            let height = heights[coord.0][coord.1];
            if height > tallest && !counted.contains(&coord) {
                visible += 1;
                counted.insert(coord);
            }
            tallest = max(tallest, height);
        }
        tallest = -1;
    }
    visible
}

// Elapsed time: 10260 us
// Memory Used: 93.625 kb
pub fn get_highest_scenic_score() -> Result<usize> {
    let input = get_input(INPUT)?;
    let heights: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut max_score = 0;
    let (n, m) = (heights.len(), heights[0].len());
    for i in 0..n {
        for j in 0..m {
            max_score = max(max_score, get_score(&heights, i, j, n, m));
        }
    }
    Ok(max_score)
}

fn get_score(heights: &Vec<Vec<usize>>, i: usize, j: usize, n: usize, m: usize) -> usize {
    let mut score = 1;
    score *= get_count_direction(heights, i, j, Either::Right((0..i).rev()), true);
    score *= get_count_direction(heights, i, j, Either::Left((i + 1)..n), true);
    score *= get_count_direction(heights, i, j, Either::Right((0..j).rev()), false);
    score *= get_count_direction(heights, i, j, Either::Left((j + 1)..m), false);
    score
}

fn get_count_direction(
    heights: &Vec<Vec<usize>>,
    i: usize,
    j: usize,
    r: Either<Range<usize>, Rev<Range<usize>>>,
    is_vertical: bool,
) -> usize {
    let mut count = 0;
    for k in r {
        count += 1;
        let coord = if is_vertical { (k, j) } else { (i, k) };
        if heights[i][j] <= heights[coord.0][coord.1] {
            break;
        };
    }
    count
}
