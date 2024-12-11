use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn in_bounds(&self, bounds: (usize, usize)) -> bool {
        self.0 < bounds.0 && self.1 < bounds.1
    }

    fn move_with_directions(&self, direction_i: i32, direction_j: i32) -> Self {
        Self(
            (self.0 as i32 + direction_i) as usize,
            (self.1 as i32 + direction_j) as usize,
        )
    }

    fn direction_i(&self, other: &Self) -> i32 {
        other.0 as i32 - self.0 as i32
    }

    fn direction_j(&self, other: &Self) -> i32 {
        other.1 as i32 - self.1 as i32
    }
}

pub fn part1(input: &str) -> u32 {
    let nodes = build_nodes(input);
    let max_i = input.lines().count();
    let max_j = input.lines().next().unwrap().len();
    let mut antinodes = HashSet::new();
    for positions in nodes {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                build_antinodes(positions[i], positions[j])
                    .into_iter()
                    .filter(|pos| pos.in_bounds((max_i, max_j)))
                    .for_each(|antinode| {
                        antinodes.insert(antinode);
                    });
            }
        }
    }
    antinodes.len() as u32
}

fn build_nodes(input: &str) -> Vec<Vec<Position>> {
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                nodes.entry(c).or_default().push(Position(i, j));
            }
        }
    }
    nodes.into_values().collect()
}

fn build_antinodes(position0: Position, position1: Position) -> [Position; 2] {
    let di = position0.direction_i(&position1);
    let dj = position0.direction_j(&position1);

    let antinode_0 = position0.move_with_directions(-di, -dj);
    let antinode_1 = position1.move_with_directions(di, dj);

    [antinode_0, antinode_1]
}

pub fn part2(input: &str) -> u32 {
    let nodes = build_nodes(input);
    let nodes_positions: HashSet<Position> = nodes.iter().flatten().cloned().collect();

    let max_i = input.lines().count();
    let max_j = input.lines().next().unwrap().len();
    let mut antinodes = HashSet::new();

    for positions in nodes {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let antinode = build_antinodes_part2(positions[i], positions[j], (max_i, max_j));
                antinodes.extend(
                    antinode
                        .into_iter()
                        .filter(|pos| !nodes_positions.contains(pos)),
                );
            }
        }
    }
    antinodes.len() as u32 + nodes_positions.len() as u32
}

fn build_antinodes_part2(
    position0: Position,
    position1: Position,
    bounds: (usize, usize),
) -> Vec<Position> {
    let mut antinodes: Vec<Position> = Vec::new();

    let di = position1.direction_i(&position0);
    let dj = position1.direction_j(&position0);

    let mut next_antinode = position0.move_with_directions(-di, -dj);
    loop {
        if next_antinode.in_bounds(bounds) {
            antinodes.push(next_antinode);
            next_antinode = next_antinode.move_with_directions(-di, -dj);
        } else {
            break;
        }
    }

    let mut next_antinode = position1.move_with_directions(di, dj);
    loop {
        if next_antinode.in_bounds(bounds) {
            antinodes.push(next_antinode);
            next_antinode = next_antinode.move_with_directions(di, dj);
        } else {
            break;
        }
    }

    antinodes
}
