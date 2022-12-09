#![allow(dead_code)]

use chrono::prelude::Local;
use peak_alloc::PeakAlloc;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let start = Local::now();
    let res = day_9::find_long_rope_locations().unwrap();
    let duration = (Local::now() - start).num_microseconds().unwrap();
    println!("Result: {:?}", res);
    println!("Elapsed time: {} us", duration);
    println!("Memory Used: {} kb", PEAK_ALLOC.peak_usage_as_kb());
}
