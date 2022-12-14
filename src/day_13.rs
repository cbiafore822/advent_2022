use crate::get_input;
use std::{cmp::Ordering, io::Result, iter::Peekable, str::Chars};

use itertools::Either;

const INPUT: &str = "inputs/day_13.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 4825 us
// Memory Used: 418.31836 kb
pub fn get_right_order() -> Result<usize> {
    let input = get_input(INPUT)?;
    let packets = Packet::new_list(input);
    let mut res = 0;
    for i in 0..packets.len() {
        let packet = &packets[i];
        let order = packet.left.cmp(&packet.right);
        if order == Ordering::Less || order == Ordering::Equal {
            res += i + 1;
        }
    }
    Ok(res)
}

// Elapsed time: 5770 us
// Memory Used: 410.9912 kb
pub fn sort_packets() -> Result<usize> {
    let mut input = get_input(INPUT)?;
    input.push_str("\n[[2]]\n[[6]]\n");
    let packets = Packet::new_list(input);
    let mut lists: Vec<&MessyList> = packets
        .iter()
        .map(|p| Vec::from([&p.left, &p.right]))
        .flatten()
        .collect();
    lists.sort_by(|a, b| a.cmp(b));
    let i = lists
        .binary_search(&&MessyList::new(&mut "[[2]]".chars().peekable()))
        .unwrap()
        + 1;
    let j = lists
        .binary_search(&&MessyList::new(&mut "[[6]]".chars().peekable()))
        .unwrap()
        + 1;
    Ok(i * j)
}

struct Packet {
    left: MessyList,
    right: MessyList,
}

impl Packet {
    fn new(left: MessyList, right: MessyList) -> Self {
        Packet { left, right }
    }

    fn new_list(list: String) -> Vec<Self> {
        let mut res = Vec::new();
        let mut lines = list.lines();
        while let Some(left) = lines.next() {
            let right = lines.next().unwrap();
            res.push(Self::new(
                MessyList::new(&mut left[1..].chars().peekable()),
                MessyList::new(&mut right[1..].chars().peekable()),
            ));
            lines.next();
        }
        res
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd)]
struct MessyList {
    list: Vec<Either<MessyList, usize>>,
}

impl MessyList {
    fn new(input: &mut Peekable<Chars>) -> Self {
        let mut curr = MessyList::default();
        while let Some(c) = input.next() {
            match c {
                '[' => curr.list.push(Either::Left(Self::new(input))),
                ']' => return curr,
                ',' => continue,
                _ => {
                    let mut val = c.to_string();
                    while let Some('0'..='9') = input.peek() {
                        val.push(input.next().unwrap());
                    }
                    curr.list.push(Either::Right(val.parse::<usize>().unwrap()));
                }
            }
        }
        curr
    }

    fn from(val: usize) -> Self {
        MessyList {
            list: Vec::from([Either::Right(val)]),
        }
    }
}

impl Ord for MessyList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut res = Ordering::Equal;
        let mut s_iter = self.list.iter();
        let mut o_iter = other.list.iter();
        while res == Ordering::Equal {
            let items = (s_iter.next(), o_iter.next());
            match items {
                (None, None) => return Ordering::Equal,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
                (Some(s_item), Some(o_item)) => {
                    res = match (s_item, o_item) {
                        (Either::Left(ml1), Either::Left(ml2)) => ml1.cmp(ml2),
                        (Either::Left(ml1), Either::Right(v2)) => ml1.cmp(&MessyList::from(*v2)),
                        (Either::Right(v1), Either::Left(ml2)) => MessyList::from(*v1).cmp(ml2),
                        (Either::Right(v1), Either::Right(v2)) => v1.cmp(v2),
                    };
                }
            };
        }
        res
    }
}
