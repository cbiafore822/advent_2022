use crate::get_input;
use std::{collections::VecDeque, io::Result};

const INPUT: &str = "inputs/day_11.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 786 us
// Memory Used: 3.9238281 kb
pub fn get_monkey_business() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut lines = input.lines();
    let mut monkies = Vec::new();
    let mut modu = 1;
    while let Some(_line) = lines.next() {
        let monkey = Monkey::new(
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        );
        modu *= monkey.test;
        monkies.push(monkey);
        lines.next();
    }
    for _i in 0..20 {
        for j in 0..monkies.len() {
            while !monkies[j].items.is_empty() {
                let monkey = monkies.get_mut(j).unwrap();
                let item = monkey.items.pop_front().unwrap();
                let (new, next) = monkey.insp_items(item, true, modu);
                monkies.get_mut(next).unwrap().items.push_back(new);
            }
        }
    }
    monkies.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    Ok(monkies[0].inspection_count * monkies[1].inspection_count)
}

// Elapsed time: 74697 us
// Memory Used: 3.9238281 kb
pub fn get_more_monkey_business() -> Result<usize> {
    let input = get_input(INPUT)?;
    let mut lines = input.lines();
    let mut monkies = Vec::new();
    let mut modu = 1;
    while let Some(_line) = lines.next() {
        let monkey = Monkey::new(
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        );
        modu *= monkey.test;
        monkies.push(monkey);
        lines.next();
    }
    for _i in 0..10000 {
        for j in 0..monkies.len() {
            while !monkies[j].items.is_empty() {
                let monkey = monkies.get_mut(j).unwrap();
                let item = monkey.items.pop_front().unwrap();
                let (new, next) = monkey.insp_items(item, false, modu);
                monkies.get_mut(next).unwrap().items.push_back(new);
            }
        }
    }
    monkies.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    Ok(monkies[0].inspection_count * monkies[1].inspection_count)
}

struct Monkey {
    items: VecDeque<usize>,
    inspection_count: usize,
    operation: fn(&Self, usize) -> usize,
    op_const: Option<usize>,
    test: usize,
    next_monkey: (usize, usize),
}

impl Monkey {
    fn new(
        starting_items: &str,
        operation: &str,
        test: &str,
        if_true: &str,
        if_false: &str,
    ) -> Self {
        let (_, start_list) = starting_items.split_once(": ").unwrap();
        let items = start_list
            .split(", ")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let (_, equation) = operation.split_once(" = ").unwrap();
        let eq_parts: Vec<&str> = equation.split(" ").collect();
        let operation = if eq_parts[1] == "+" {
            Monkey::add
        } else {
            Monkey::mult
        };
        let op_const = eq_parts[2].parse::<usize>().ok();
        let test = test[21..].parse::<usize>().unwrap();
        let next_monkey = (
            if_true[29..].parse::<usize>().unwrap(),
            if_false[30..].parse::<usize>().unwrap(),
        );
        Monkey {
            items,
            inspection_count: 0,
            operation,
            op_const,
            test,
            next_monkey,
        }
    }

    fn insp_items(&mut self, item: usize, relief: bool, modu: usize) -> (usize, usize) {
        let mut item = (self.operation)(self, item);
        if relief {
            item /= 3;
        }
        item = item % modu;
        self.inspection_count += 1;
        (
            item,
            if item % self.test == 0 {
                self.next_monkey.0
            } else {
                self.next_monkey.1
            },
        )
    }

    fn add(&self, old: usize) -> usize {
        old + self.op_const.unwrap_or(old)
    }

    fn mult(&self, old: usize) -> usize {
        old * self.op_const.unwrap_or(old)
    }
}
