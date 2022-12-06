#![allow(dead_code)]

use chrono::prelude::Local;
use peak_alloc::PeakAlloc;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let start = Local::now();
    let res = day_5::get_tops_2();
    let duration = (Local::now() - start).num_microseconds().unwrap();
    println!(
        "Result: {:?}",
        res.unwrap_or_else(|err| {
            println!("{:?}", err);
            "Error".to_string()
        })
    );
    println!("Elapsed time: {} us", duration);
    println!("Memory Used: {} kb", PEAK_ALLOC.peak_usage_as_kb());
}
