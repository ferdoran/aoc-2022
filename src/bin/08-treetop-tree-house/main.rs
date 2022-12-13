use std::fs;

const GRID_SIZE: usize = 99;

fn main() {
    let input = fs::read_to_string("./src/bin/08-treetop-tree-house/input.txt");
    let input_string = match input {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read input file: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = input_string.lines().collect();
    let trees: Vec<u32> = lines.iter()
        .flat_map(|l| l.chars())
        .map(|c|c.to_digit(10).unwrap())
        .collect();
    let visible_trees = part1_visible_trees(&trees);
    let scenic_score = part2_scenic_score(&trees);

    println!("Part 1: {}", visible_trees + 4 * (GRID_SIZE-1));
    println!("Part 2: {}", scenic_score);
}

fn part1_visible_trees(trees: &Vec<u32>) -> usize {
    let mut visible_trees = 0;
    for x in 1..GRID_SIZE - 1 {
        for y in 1..GRID_SIZE - 1 {
            // inner grid trees
            if is_visible_from_outside(&trees, x, y) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn part2_scenic_score(trees: &Vec<u32>) -> u32 {
    let mut scenic_scores: Vec<u32> = Vec::new();
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            // inner grid trees
            scenic_scores.push(scenic_score(trees, x, y));
        }
    }

    return scenic_scores.iter().max().map(|a| *a).unwrap();
}

fn is_visible_from_outside(trees: &Vec<u32>, x: usize, y: usize) -> bool {
    let t = trees[y*GRID_SIZE+x];
    let (left, right) = get_trees_in_row(trees, x, y);
    let (down, up) = get_trees_in_col(trees, x, y);

    left.iter().all(|_t| *_t < t)
        || right.iter().all(|_t| *_t < t)
        || down.iter().all(|_t| *_t < t)
        || up.iter().all(|_t| *_t < t)
}

fn scenic_score(trees: &Vec<u32>, x: usize, y: usize) -> u32 {
    let (left, right) = scenic_score_row(trees, x, y);
    let (down, up) = scenic_score_col(trees, x, y);

    left * right * down * up
}

fn get_trees_in_row(trees: &Vec<u32>, _x: usize, y: usize) -> (Vec<u32>, Vec<u32>) {
    let mut lower: Vec<u32> = Vec::new();
    let mut upper: Vec<u32> = Vec::new();
    for x in 0..GRID_SIZE {
        let t = trees[y * GRID_SIZE + x];

        if x < _x {
            lower.push(t);
        } else if x > _x {
            upper.push(t);
        }
    }

    return (lower, upper);
}

fn get_trees_in_col(trees: &Vec<u32>, x: usize, _y: usize) -> (Vec<u32>, Vec<u32>) {
    let mut lower: Vec<u32> = Vec::new();
    let mut upper: Vec<u32> = Vec::new();
    for y in 0..GRID_SIZE {
        let t = trees[y * GRID_SIZE + x];

        if y < _y {
            lower.push(t);
        } else if y > _y {
            upper.push(t);
        }
    }

    return (lower, upper);
}

fn scenic_score_row(trees: &Vec<u32>, x: usize, y: usize) -> (u32, u32) {
    let mut trees_left = 0;
    let mut trees_right = 0;

    let t = trees[y * GRID_SIZE + x];
    for x1 in (0..x).rev() {
        let t_a = trees[y * GRID_SIZE + x1];
        trees_left += 1;
        if t_a >= t {
            break;
        }
    }

    for x1 in x+1..GRID_SIZE {
        let t_a = trees[y * GRID_SIZE + x1];
        trees_right += 1;
        if t_a >= t {
            break;
        }
    }

    return (trees_left, trees_right);
}

fn scenic_score_col(trees: &Vec<u32>, x: usize, y: usize) -> (u32, u32) {
    let mut trees_down = 0;
    let mut trees_up = 0;

    let t = trees[y * GRID_SIZE + x];
    for y1 in (0..y).rev() {
        let t_a = trees[y1 * GRID_SIZE + x];
        trees_down += 1;
        if t_a >= t {
            break;
        }
    }

    for y1 in y+1..GRID_SIZE {
        let t_a = trees[y1 * GRID_SIZE + x];
        trees_up += 1;
        if t_a >= t {
            break;
        }
    }

    return (trees_down, trees_up);
}

