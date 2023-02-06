use std::{io::{BufRead, Lines, Error}, ops::{Range, RangeBounds}};

pub(crate) fn count_overlapping_ranges<T:Iterator<Item = Result<String, Error>>>(iterator: &mut T) -> usize {
    let bools = iterator.filter_map(|line| line.ok().and_then(is_overlapping_ranges));
    return bools.filter(|b| *b).count()
}

fn is_overlapping_ranges(two_range_line: String) -> Option<bool> {
    let ranges: Vec<Range<u32>> = two_range_line
        .split(',')
        .map(string_to_range)
        .flatten()
        .collect();
    if ranges.len() == 2 {
        return Some(ranges_overlap(&ranges[0], &ranges[1]))
    } else {
        return None
    }
}

fn ranges_overlap(range1: &Range<u32>, range2: &Range<u32>) -> bool {
    (range1.contains(&range2.start) || range1.contains(&(range2.end - 1))) ||
    (range2.contains(&range1.start) || range2.contains(&(range1.end - 1)))
}

fn string_to_range(string_range: &str) -> Option<Range<u32>> {
    let ints: Vec<u32> = string_range
        .split('-')
        .map(|n| n.parse::<u32>().ok())
        .flatten()
        .collect();
    if ints.len() == 2 {
        return Some(ints[0]..ints[1]+1)
    } else {
        return  None
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn find_common_items_with_sample_input() {
        let input = SAMPLE_INPUT.to_string(); 
        let buf = BufReader::new(input.as_bytes());
        let overlapping_range_count = count_overlapping_ranges(&mut buf.lines());
        assert_eq!(overlapping_range_count, 4);
    }
}