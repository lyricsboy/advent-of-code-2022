use std::io::{self, BufRead};

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let stdin = io::stdin();
    let overlapping_range_count = day04::cleanup::count_overlapping_ranges(&mut stdin.lock().lines());
    println!("Overlapping range count: {:?}", overlapping_range_count);
}
