use std::collections::HashSet;

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .chars()
        .map(|c| match c {
            '^' => NORTH,
            'v' => SOUTH,
            '>' => EAST,
            '<' => WEST,
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[(isize, isize)]) -> u64 {
    let mut current_position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(current_position);
    for (dx, dy) in input {
        current_position = (current_position.0 + dx, current_position.1 + dy);
        visited.insert(current_position);
    }
    visited.len() as u64
}

pub fn part2(input: &[(isize, isize)]) -> usize {
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    let mut visited = HashSet::new();

    visited.insert(santa);

    for (i, (dx, dy)) in input.iter().enumerate() {
        if i % 2 == 0 {
            santa = (santa.0 + dx, santa.1 + dy);
            visited.insert(santa);
        } else {
            robo_santa = (robo_santa.0 + dx, robo_santa.1 + dy);
            visited.insert(robo_santa);
        }
    }
    visited.len()
}
