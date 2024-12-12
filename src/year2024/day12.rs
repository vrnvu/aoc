use std::collections::HashSet;

use crate::utils::grid::{Grid, Vector, DOWN, LEFT, RIGHT, UP};

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::new(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

pub fn part1(input: &Grid<u8>) -> usize {
    let mut visited = HashSet::new();
    let mut result = 0;
    for i in 0..input.len() {
        for j in 0..input.width() {
            let position = Vector::new(i, j);

            if visited.contains(&position) {
                continue;
            }

            let (area, perimeter) = visit_part1(&mut visited, input, position);
            result += area * perimeter;
        }
    }
    result
}

pub fn part2(input: &Grid<u8>) -> usize {
    let mut visited: HashSet<Vector> = HashSet::new();
    let mut result = 0;
    for i in 0..input.len() {
        for j in 0..input.width() {
            let position = Vector::new(i, j);

            if visited.contains(&position) {
                continue;
            }

            let (area, sides) = visit_part2(&mut visited, input, position);
            result += area * sides;
        }
    }
    result
}

fn visit_part1(
    visited: &mut HashSet<Vector>,
    input: &Grid<u8>,
    position: Vector,
) -> (usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;

    let value = input.get(position).unwrap();
    let mut stack = vec![position];
    while let Some(current_position) = stack.pop() {
        if !visited.insert(current_position) {
            continue;
        }
        area += 1;

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next = current_position + dir;
            match input.get(next) {
                None => perimeter += 1,
                Some(next_value) => {
                    if next_value == value {
                        stack.push(next);
                    } else {
                        perimeter += 1;
                    }
                }
            }
        }
    }
    (area, perimeter)
}

fn visit_part2(visited: &mut HashSet<Vector>, input: &Grid<u8>, start: Vector) -> (usize, usize) {
    let mut area = 0;
    let mut corners = 0;

    let mut stack = vec![start];
    let value = input.get(start).unwrap();
    while let Some(current_position) = stack.pop() {
        if !visited.insert(current_position) {
            continue;
        }
        area += 1;

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next = current_position + dir;
            if let Some(v) = input.get(next) {
                if *v == *value {
                    stack.push(next);
                }
            }
        }

        let corner_checks = [
            (
                current_position + UP,
                current_position + RIGHT,
                current_position + UP + RIGHT,
            ),
            (
                current_position + UP,
                current_position + LEFT,
                current_position + UP + LEFT,
            ),
            (
                current_position + DOWN,
                current_position + RIGHT,
                current_position + DOWN + RIGHT,
            ),
            (
                current_position + DOWN,
                current_position + LEFT,
                current_position + DOWN + LEFT,
            ),
        ];

        for (n1, n2, diagonal) in corner_checks {
            let n1_match = input.get(n1).map_or(false, |v| v == value);
            let n2_match = input.get(n2).map_or(false, |v| v == value);
            let diagonal_match = input.get(diagonal).map_or(false, |v| v == value);

            if (!n1_match && !n2_match) || (n1_match && n2_match && !diagonal_match) {
                corners += 1;
            }
        }
    }
    (area, corners)
}