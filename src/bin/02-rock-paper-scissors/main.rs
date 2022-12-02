use std::fs;
use crate::Outcome::{Draw, Lose, Win};
use crate::Shape::{Paper, Rock, Scissors};

enum Shape {
    Rock,
    Paper,
    Scissors,
    None
}

enum Outcome {
    Win,
    Draw,
    Lose
}

fn main() {
    let input = fs::read_to_string("./src/bin/02-rock-paper-scissors/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        },
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let scores = lines.iter().map(|l| {
        let plays: Vec<&str> = l.split(" ").take(2).collect();
        (plays[0], plays[1])
    }).map(|(enemy, outcome)| {
        let enemy_shape = match enemy {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => Shape::None
        };

        let part_1_shape = part1_shape(&outcome);
        let part_2_shape = part2_shape(&enemy_shape, outcome);

        (enemy_shape, (part_1_shape, part_2_shape))
    }).map(|(enemy, (p1_shape, p2_shape))| {

        let p1_score = score_for_shape(&p1_shape) + score_for_outcome(&enemy, &p1_shape);
        let p2_score = score_for_shape(&p2_shape) + score_for_outcome(&enemy, &p2_shape);

        return (p1_score, p2_score);
    }).collect::<Vec<(u32, u32)>>();

    let p1_score: u32 = scores.iter().map(|(p1, _)| p1).sum();
    let p2_score: u32 = scores.iter().map(|(_, p2)| p2).sum();

    println!("Part 1: Total Score = {}", p1_score);
    println!("Part 2: Total Score = {}", p2_score);
}

fn part1_shape(s: &str) -> Shape {
     match s {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => Shape::None
    }
}

fn part2_shape(enemy_shape: &Shape, s: &str) -> Shape {
    let outcome = match s {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => Lose,
    };

    return shape_for_desired_outcome(enemy_shape, &outcome);
}

fn shape_for_desired_outcome(s: &Shape, desired_outcome: &Outcome) -> Shape {
    return match *s {
        Rock => match *desired_outcome {
            Win => Paper,
            Draw => Rock,
            Lose => Scissors
        }
        Paper => match *desired_outcome {
            Win => Scissors,
            Draw => Paper,
            Lose => Rock
        }
        Scissors => match *desired_outcome {
            Win => Rock,
            Draw => Scissors,
            Lose => Paper
        }
        Shape::None => Shape::None
    }
}

fn score_for_shape(s: &Shape) -> u32 {
    return match s {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
        _ => 0
    }
}


fn score_for_outcome(enemy: &Shape, me: &Shape) -> u32 {
    return match enemy {
        Rock => match me {
            Paper => 6,
            Scissors => 0,
            Rock => 3,
            _ => 0
        }
        Paper => match me {
            Scissors => 6,
            Rock => 0,
            Paper => 3,
            _ => 0
        }
        Scissors => match me {
            Rock => 6,
            Paper => 0,
            Scissors => 3,
            _ => 0
        },
        _ => 0
    };
}