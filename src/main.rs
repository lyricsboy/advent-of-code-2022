use std::io::{self, BufRead};

mod day01;
mod day02;
mod day03;

fn main() {
    let stdin = io::stdin();
    let priority_sum = day03::rucksack::priority_sum_from_lines(&mut stdin.lock().lines());
    println!("Priority Sum: {:?}", priority_sum);
}
