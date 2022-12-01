use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/bin/01-calorie-counting/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        },
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let mut calories_per_elf_sorted = calc_calories_per_elf(lines);

    let max_calories = calories_per_elf_sorted.iter().max().unwrap();
    println!("Part 1: Max Calories = {:?}", max_calories);

    let top_3_calories: i32 = calories_per_elf_sorted.iter().rev().take(3).sum();
    println!("Part 2: Calories of top 3 = {:?}", top_3_calories);
}

fn calc_calories_per_elf(calories_lines: Vec<&str>) -> Vec<i32> {
    let empty_line_pattern = Regex::new(r"^\s*$").unwrap();
    let mut calories_per_elf: Vec<i32> = Vec::new();
    let mut calorie_counter = 0;
    calories_lines.iter().enumerate().for_each(|(line_num, line)| {
        if empty_line_pattern.is_match(line) {
            calories_per_elf.push(calorie_counter);
            calorie_counter = 0;
        } else {
            let n = line.parse::<i32>().expect(&*format!("Line {}: '{}' is an integer", line_num, line));
            calorie_counter += n;
        }
    });

    calories_per_elf.sort();
    return calories_per_elf;
}