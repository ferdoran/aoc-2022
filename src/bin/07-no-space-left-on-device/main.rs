use std::collections::{HashMap};
use std::fs;
use std::path::PathBuf;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/bin/07-no-space-left-on-device/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let cd_pattern = Regex::new(r"^\$\scd\s(?P<dir>.+)$").unwrap();
    let file_pattern = Regex::new(r"^(?P<size>\d+)\s(?P<name>.+)$").unwrap();
    let dir_pattern = Regex::new(r"^dir\s(?P<name>\w+)$").unwrap();

    let mut pb = PathBuf::new();
    let mut entries : HashMap<PathBuf, usize> = HashMap::new();
    entries.insert(PathBuf::from("/"), 0);

    lines.iter().for_each(|l| {
        if cd_pattern.is_match(l) {
            let p = cd_pattern.captures(l).unwrap();
            let dir = p.name("dir").map(|m| m.as_str()).unwrap();
            match dir {
                ".." => {
                    pb.pop();
                },
                "/" => {
                    pb = PathBuf::from(dir);
                }
                _ => {
                    pb.push(dir);
                }
            };
        } else if file_pattern.is_match(l) {
            let captures = file_pattern.captures(l).unwrap();
            let size = captures.name("size").map(|s| s.as_str().parse::<usize>().unwrap()).unwrap();
            let name = captures.name("name").map(|s| s.as_str()).unwrap();
            let mut p = pb.clone();
            p.push(name);
            entries.insert(p, size);
        } else if dir_pattern.is_match(l) {
            let captures = dir_pattern.captures(l).unwrap();
            let name = captures.name("name").map(|s| s.as_str()).unwrap();
            let mut p = pb.clone();
            p.push(name);
            entries.insert(p, 0);
        }
    });

    let mut entries_clone = entries.clone();
    entries.iter_mut()
        .filter(|(path, _) | path.extension().is_none())
        .for_each(|(dir_path, size)| {
            let sum: usize = entries_clone.iter()
                .filter(|(p, s)| **s > 0 && p.parent().unwrap().starts_with(dir_path))
                .map(|(_, s)| *s)
                .sum();
            *size = sum;
        });

    entries_clone = entries.clone();
    entries.iter_mut()
        .filter(|(path, _) | path.extension().is_some())
        .for_each(|(dir_path, size)| {
            let sum: usize = entries_clone.iter()
                .filter(|(p, _)| p.parent().is_some() && p.parent().unwrap().starts_with(dir_path) && p.extension().is_none())
                .map(|(_, s)| *s)
                .sum();
            *size += sum;
        });


    let p1_size: usize = entries.iter()
        .filter(|(p, s)| p.extension().is_none() && **s <= 100_000)
        .filter_map(|(p, _)| {
            entries.iter()
                .filter(|(p1, _)| p1.starts_with(p))
                .min_by_key(|(p1, _)| p1.components().count())
        })
        .map(|(_, s)| *s)
        .sum();


    println!("Part 1: {}", p1_size);

    let root_size = entries[&PathBuf::from("/")];

    let free_size = 70_000_000 - root_size;
    let cleanup_size = 30_000_000 - free_size;

    let cleanup_options: Vec<(&PathBuf, &usize)> = entries.iter()
        .filter(|(_, s)| **s >= cleanup_size)
        .filter(|(p, _)| p.components().count() > 1)
        .collect();

    println!("root size: {}", root_size);
    println!("cleanup options: {:?}", cleanup_options);
    let directory_to_clean = cleanup_options.iter().min_by_key(|(_, s)| **s).unwrap();
    let (dir, size) = directory_to_clean;
    println!("Part 2: dir to clean: {}", dir.display());
    println!("Part 2: size of dir to clean: {}", size);
}

