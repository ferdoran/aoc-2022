use std::collections::HashSet;
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/bin/06-tuning-trouble/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let stream = lines[0];

    let p1_packet_start = unique_sequence_after(stream, 4);
    let p2_message_start = unique_sequence_after(stream, 14);

    println!("Part 1: first packet after {} characters", p1_packet_start);
    println!("Part 2: first message after {} characters", p2_message_start);
}

fn unique_sequence_after(stream: &str, len: usize) -> usize {
    for i in 0..stream.len() - len {
        let sequence = &stream[i..i + len];
        let chars: HashSet<char> = sequence.chars().collect();
        if chars.len() == len {
            return i+len;
        }
    }

    return 0;
}