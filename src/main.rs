use std::io::{self, BufRead};

mod day01;

fn main() {
    let stdin = io::stdin();
    let max_calories = day01::calories::max_calories(&mut stdin.lock().lines());
    println!("Max Calories: {:?}", max_calories);
}
