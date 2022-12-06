use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/bin/05-supply-stacks/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let mut splits = lines.split(|l| l.is_empty());
    let stack_defs = splits.next().unwrap();
    let commands = splits.next().unwrap().to_vec();

    let num_stacks = (stack_defs[0].len() - (stack_defs[0].len() % 3)) / 3 - 1;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new())
    }

    println!("There are {} stacks", num_stacks);
    for i in 0..stack_defs.len() - 1 {
        let stack_line: Vec<char> = stack_defs[i].chars().collect();
        stack_line.chunks(4).enumerate().for_each(|(c, chunk)| {
            if let Some(cargo) = chunk.get(1) {
                if !cargo.is_whitespace() {
                    if let Some(stack) = stacks.get_mut(c) {
                        stack.push(*cargo);
                    }
                }
            }
        });
    }

    stacks.iter_mut()
        .map(|s| {s.reverse(); s})
        .enumerate()
        .for_each(|(i, s)| {
            println!("Stack {}: {:?}", i + 1, s);
        });

    process_movements(&mut stacks, &commands);

    let tops = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("Tops of the stacks: {}", tops);
}

fn process_movements(stacks: &mut Vec<Vec<char>>, commands: &Vec<&str>) {
    let command_pattern = Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
    commands.iter().map(|command| {
        let captures = command_pattern.captures(command).unwrap();
        let amount = captures.name("amount")
            .map(|m| m.as_str().parse::<usize>())
            .unwrap()
            .unwrap();
        let from = captures.name("from")
            .map(|m| m.as_str().parse::<usize>())
            .unwrap()
            .unwrap();
        let to = captures.name("to")
            .map(|m| m.as_str().parse::<usize>())
            .unwrap()
            .unwrap();

        (amount, from-1, to-1)
    }).for_each(|(amount, from, to)| {
        let from_stack = stacks.get_mut(from).unwrap();
        let mut popped: Vec<char> = Vec::new();
        for _ in 0..amount {
            let cargo = from_stack.pop().unwrap();
            popped.push(cargo);
        }
        // popped.reverse(); // don't reverse for part 2 to maintain order

        let to_stack = stacks.get_mut(to).unwrap();
        for _ in 0..amount {
            let cargo = popped.pop().unwrap();
            to_stack.push(cargo);
        }
    });
}