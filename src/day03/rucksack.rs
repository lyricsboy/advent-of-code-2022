use std::{io::{BufRead, Lines, Error}, collections::HashSet};
use itertools::Itertools;

fn find_common_items<T:BufRead>(iterator: &mut Lines<T>) -> Vec<char> {
    iterator.flat_map(|line| 
        find_common_item(line.unwrap())
    ).collect()
}

fn find_common_item_between_rucksacks<T:Iterator<Item = Result<String, Error>>>(iterator: &mut T) -> Option<char> {
    iterator.map(|line| HashSet::<_>::from_iter(line.unwrap().chars()))
        .reduce(|intersection, set| intersection.intersection(&set).cloned().collect())
        .map(|maybe_set| maybe_set.into_iter().nth(0))
        .flatten()
}

fn find_common_item(rucksack_contents: String) -> Option<char> {
    let (compartment1_str, compartment2_str) = rucksack_contents.split_at(rucksack_contents.len() / 2);
    let compartment1_set = HashSet::<_>::from_iter(compartment1_str.chars());
    let compartment2_set = HashSet::<_>::from_iter(compartment2_str.chars());
    let mut common_items = compartment1_set.intersection(&compartment2_set);
    common_items.nth(0).cloned()
}

fn priority_sum(items: Vec<char>) -> u32 {
    items.iter().map(|item| {
        if item.is_ascii_uppercase() {
            (*item as u32) - ('A' as u32) + 27
        } else {
            (*item as u32) - ('a' as u32) + 1
        }
    }).sum()
}

pub(crate) fn priority_sum_from_lines<T:BufRead>(lines: &mut Lines<T>) -> u32 {
    priority_sum(find_common_items(lines))
}

pub(crate) fn priority_sum_of_common_items<T:Iterator<Item = Result<String, Error>>>(iterator: &mut T) -> u32 {
    let common_items = iterator.chunks(3).into_iter()
        .flat_map(|mut chunk| find_common_item_between_rucksacks(&mut chunk))
        .collect();
    return priority_sum(common_items);
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn find_common_items_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let common_items = find_common_items(&mut buf.lines());
        assert_eq!(common_items, ['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn priority_sum_with_sample_input() {
        assert_eq!(priority_sum(['p', 'L', 'P', 'v', 't', 's'].to_vec()), 157)
    }

    #[test]
    fn find_common_item_between_rucksacks_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let mut buf = BufReader::new(input.as_bytes());
        let mut first3 = buf.lines().take(3);
        let mut common_item = find_common_item_between_rucksacks(&mut first3);
        assert_eq!(common_item, Some('r'));
        // recreating the buf to avoid complaints about move/copy semantics
        buf = BufReader::new(input.as_bytes());
        let mut second3 = buf.lines().skip(3).take(3);
        common_item = find_common_item_between_rucksacks(&mut second3);
        assert_eq!(common_item, Some('Z'));
    }

    #[test]
    fn find_priority_sum_of_common_items() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let sum = priority_sum_of_common_items(&mut buf.lines());
        assert_eq!(sum, 70);
    }
}