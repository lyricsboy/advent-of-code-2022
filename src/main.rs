use std::io::{self, BufRead};

mod day01;
mod day02;
mod day03;

fn main() {
    let stdin = io::stdin();
    let common_item_sum = day03::rucksack::priority_sum_of_common_items(&mut stdin.lock().lines());
    println!("Common item sum: {:?}", common_item_sum);
}
