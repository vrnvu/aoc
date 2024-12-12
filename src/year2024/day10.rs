use std::collections::HashSet;

use crate::utils::grid::{Grid, Vector, DOWN, LEFT, RIGHT, UP};

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Grid<u8> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part1(input: &Grid<u8>) -> usize {
    let mut trailheads = 0;
    for (origin, &value) in input.iter_positions() {
        if value == 0 {
            trailheads += count_trailheads(input, origin);
        }
    }
    trailheads
}

fn count_trailheads(grid: &Grid<u8>, origin: Vector) -> usize {
    let mut trailheads = 0;
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    stack.push(origin);
    while let Some(position) = stack.pop() {
        if !visited.insert(position) {
            continue;
        }

        if let Some(value) = grid.get(position) {
            if *value == 9 {
                trailheads += 1;
                continue;
            }

            let directions = [UP, DOWN, LEFT, RIGHT];
            for direction in directions {
                if let Some(next_value) = grid.get_from(position, direction) {
                    if *next_value == *value + 1 {
                        stack.push(position + direction);
                    }
                }
            }
        }
    }

    trailheads
}

pub fn part2(input: &Grid<u8>) -> usize {
    let mut trailheads = 0;
    for (position, &value) in input.iter_positions() {
        if value == 0 {
            trailheads += count_all_trailheads(input, position);
        }
    }
    trailheads
}

fn count_all_trailheads(input: &Grid<u8>, origin: Vector) -> usize {
    let mut all_trailheads = 0;
    let mut stack = Vec::new();
    stack.push(origin);

    while let Some(position) = stack.pop() {
        if let Some(value) = input.get(position) {
            if *value == 9 {
                all_trailheads += 1;
                continue;
            }

            let directions = [UP, DOWN, LEFT, RIGHT];
            for direction in directions {
                if let Some(next_value) = input.get_from(position, direction) {
                    if *next_value == *value + 1 {
                        stack.push(position + direction);
                    }
                }
            }
        }
    }

    all_trailheads
}
