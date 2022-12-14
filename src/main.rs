use std::io::{self, BufRead};

mod day01;

fn main() {
    let stdin = io::stdin();
    let top_three_total = day01::calories::top_three_total(&mut stdin.lock().lines());
    println!("Top 3 total: {:?}", top_three_total);
}
