use std::collections::HashMap;
use std::fs;


fn main() {
    let input = fs::read_to_string("./src/bin/03-rucksack-reorganisation/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();

    part_one(lines.clone());
    part_two(lines);
}

fn part_two(lines: Vec<&str>) {
    let sum: u32 = lines.chunks(3).map(|group| {
        group[0].chars().find(|c| {
            group.iter().all(|g| g.contains(*c))
        }).unwrap()
    }).map(|c| c as u32)
        .map(priority_for_item)
        .sum();

    println!("Part 2: Sum = {}", sum);
}

fn part_one(lines: Vec<&str>) {
    let p1_sum: u32 = lines.iter().map(|l| {
        l.split_at(l.len() / 2)
    }).map(|(first, second)| {
        first.chars().find(|c| second.contains(*c)).map(|c| c as u32).unwrap()
    })
        .map(priority_for_item)
        .sum();

    println!("Part 1: Sum = {:?}", p1_sum);
}

fn priority_for_item(item: u32) -> u32 {
    match item {
        65..=90 => item - 38,
        97..=122 => item - 96,
        _ => 0
    }
}
