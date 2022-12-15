use std::io::{self, BufRead};

mod day01;
mod day02;

fn main() {
    let stdin = io::stdin();
    let rps_score = day02::rps::rps_score(&mut stdin.lock().lines());
    println!("RPS Score: {:?}", rps_score);
}
