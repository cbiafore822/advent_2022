use crate::get_input;
use regex::Regex;
use std::{cmp::max, collections::HashSet, io::Result};

const INPUT: &str = "inputs/day_15.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 2872 us
// Memory Used: 60.625 kb
pub fn find_bad_beacons_locs() -> Result<isize> {
    let mut res = 0;
    let tunnel_map = TunnelMap::new(get_input(INPUT)?);
    let ranges = tunnel_map.find_bad_locs(2000000);
    let mut ranges_iter = ranges.iter();
    let mut curr = *ranges_iter.next().unwrap_or(&(0, 0));
    while let Some(other) = ranges_iter.next() {
        if curr.1 >= other.0 {
            curr.1 = max(curr.1, other.1);
        } else {
            res += curr.1 - curr.0;
            curr = *other;
        }
    }
    res += curr.1 - curr.0;
    Ok(res)
}

// Elapsed time: 6529660 us
// Memory Used: 60.625 kb
pub fn find_frequency() -> Result<isize> {
    let tunnel_map = TunnelMap::new(get_input(INPUT)?);
    for i in 0..4000000 {
        let mut curr = 0;
        for (s, e) in tunnel_map.find_bad_locs(i) {
            if curr + 2 == s {
                return Ok(((curr + 1) * 4000000) + i);
            } else {
                curr = max(curr, e);
            }
        }
    }
    panic!("Did not find missing frequency");
}

#[derive(Debug)]
struct TunnelMap {
    sensors: Vec<((isize, isize), (isize, isize))>,
    beacons: HashSet<(isize, isize)>,
}

impl TunnelMap {
    fn new(input: String) -> Self {
        let mut sensors = Vec::new();
        let mut beacons = HashSet::new();
        let re = Regex::new(r"-?\d+").unwrap();
        let mut locations = re.find_iter(&input);
        while let (Some(sx), Some(sy), Some(bx), Some(by)) = (
            locations.next(),
            locations.next(),
            locations.next(),
            locations.next(),
        ) {
            let sensor = (
                sx.as_str().parse::<isize>().unwrap(),
                sy.as_str().parse::<isize>().unwrap(),
            );
            let beacon = (
                bx.as_str().parse::<isize>().unwrap(),
                by.as_str().parse::<isize>().unwrap(),
            );
            sensors.push((sensor, beacon));
            beacons.insert(beacon);
        }
        TunnelMap { sensors, beacons }
    }

    fn find_bad_locs(&self, row: isize) -> Vec<(isize, isize)> {
        let mut ranges: Vec<(isize, isize)> = self
            .sensors
            .iter()
            .filter_map(|pair| {
                let (sx, sy) = pair.0;
                let (bx, by) = pair.1;
                let (dx, dy) = ((bx - sx).abs(), (by - sy).abs());
                let remaining = dx + dy - (row - sy).abs();
                if remaining < 0 {
                    return None;
                }
                Some((sx - remaining, sx + remaining))
            })
            .collect();
        ranges.sort();
        ranges
    }
}
