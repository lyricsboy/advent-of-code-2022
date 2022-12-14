use std::io::{Lines};
use std::io::prelude::*;

pub(crate) fn max_calories<T:BufRead>(iterator: &mut Lines<T>) -> Option<i32> {
    let mut calorie_counts: Vec<i32> = Vec::new();
    while let Some(calorie_count) = count_calories(iterator) {
        calorie_counts.push(calorie_count);
    }
    return calorie_counts.iter().max().cloned();
}

fn count_calories<T:BufRead>(iterator: &mut Lines<T>) -> Option<i32> {
    iterator.map_while(|line| {
        line.unwrap().parse::<i32>().ok()
    }).reduce(|sum, calories| sum + calories)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn count_calories_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let calorie_count = count_calories(&mut buf.lines());
        assert_eq!(calorie_count, Some(6000));
    }

    #[test]
    fn max_calories_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let max_calories = max_calories(&mut buf.lines());
        assert_eq!(max_calories, Some(24000));
    }
}