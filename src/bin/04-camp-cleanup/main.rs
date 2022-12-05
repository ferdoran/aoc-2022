use std::collections::HashMap;
use std::fs;
use std::iter::Filter;
use std::ops::RangeInclusive;


fn main() {
    let input = fs::read_to_string("./src/bin/04-camp-cleanup/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let ranges: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = lines.iter()
        .map(|l| {
            let (first, second) = l.split_once(",").unwrap();
            let first_range = range_from_str(first);
            let second_range = range_from_str(second);
            (first_range, second_range)
        }).collect();

    let p1_contained_pairs = ranges.iter()
        .filter(part_one_filter)
        .count();
    let p2_overlaps = ranges.iter()
        .filter(part_two_filter)
        .count();

    println!("Part 1: # of pairs fully containing the other sections = {}", p1_contained_pairs);
    println!("Part 1: # of pairs overlapping = {}", p2_overlaps);
}

fn range_from_str(s: &str) -> RangeInclusive<u32> {
    let (start, end) = s.split_once("-")
        .map(|(s, e)| (s.parse::<u32>().unwrap(), e.parse::<u32>().unwrap()))
        .unwrap();
    return start..=end
}

fn part_one_filter(ranges: &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    let (a, b) = ranges;
    a.contains(b.start()) && a.contains(b.end())
        || b.contains(a.start()) && b.contains(a.end())
}

fn part_two_filter(ranges: &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    let (a, b) = ranges;
    a.contains(b.start())
    || a.contains(b.end())
    || b.contains(a.start())
    || b.contains(a.end())
}