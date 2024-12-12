use std::collections::HashSet;

use crate::utils::grid::{Vector, DOWN, LEFT, RIGHT, UP};

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Vector> {
    input
        .chars()
        .map(|c| match c {
            '^' => UP,
            'v' => DOWN,
            '>' => RIGHT,
            '<' => LEFT,
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[Vector]) -> u64 {
    let mut current_position = Vector::new(0, 0);
    let mut visited = HashSet::new();
    visited.insert(current_position);
    for &direction in input {
        current_position = current_position + direction;
        visited.insert(current_position);
    }
    visited.len() as u64
}

pub fn part2(input: &[Vector]) -> usize {
    let mut santa = Vector::new(0, 0);
    let mut robo_santa = Vector::new(0, 0);
    let mut visited = HashSet::new();

    visited.insert(santa);

    for (i, &direction) in input.iter().enumerate() {
        if i % 2 == 0 {
            santa = santa + direction;
            visited.insert(santa);
        } else {
            robo_santa = robo_santa + direction;
            visited.insert(robo_santa);
        }
    }
    visited.len()
}
