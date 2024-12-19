use crate::utils::grid::{Grid, Vector};

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

type Position = Vector;
type Velocity = Vector;

pub fn parse(input: &str) -> (Grid<u16>, Vec<(Position, Velocity)>) {
    let mut line_iter = input.lines();
    let (width, height) = line_iter.next().unwrap().split_once(" ").unwrap();
    let width: usize = width.parse().unwrap();
    let height: usize = height.parse().unwrap();

    let grid = Grid::new_with_size(height, width, Default::default());

    let positions = line_iter
        .map(|line| line.split_once(" ").unwrap())
        .map(|(position, velocity)| {
            let position: Vector = position.parse().unwrap();
            let velocity: Vector = velocity.parse().unwrap();
            (position.flip(), velocity.flip())
        })
        .collect();

    (grid, positions)
}

pub fn part1(input: &(Grid<u16>, Vec<(Position, Velocity)>)) -> usize {
    0
}

pub fn part2(input: &(Grid<u16>, Vec<(Position, Velocity)>)) -> usize {
    0
}
