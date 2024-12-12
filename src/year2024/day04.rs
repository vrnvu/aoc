use crate::utils::grid::{
    Grid, Vector, DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP, UP_LEFT, UP_RIGHT,
};

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Grid<char> {
    Grid::new(input.lines().map(|line| line.chars().collect()).collect())
}

pub fn part1(input: &Grid<char>) -> u32 {
    let mut count = 0;
    for (origin, &c) in input.iter_positions() {
        if c == 'X' {
            if let Some(count_found) = search_mas(input, origin) {
                count += count_found;
            }
        }
    }
    count
}

fn search_mas(grid: &Grid<char>, origin: Vector) -> Option<u32> {
    let mut count = 0;
    let vectors = [
        [UP, UP * 2, UP * 3],
        [UP_RIGHT, UP_RIGHT * 2, UP_RIGHT * 3],
        [UP_LEFT, UP_LEFT * 2, UP_LEFT * 3],
        [RIGHT, RIGHT * 2, RIGHT * 3],
        [LEFT, LEFT * 2, LEFT * 3],
        [DOWN_RIGHT, DOWN_RIGHT * 2, DOWN_RIGHT * 3],
        [DOWN_LEFT, DOWN_LEFT * 2, DOWN_LEFT * 3],
        [DOWN, DOWN * 2, DOWN * 3],
    ];
    for vector in vectors {
        let mas = vector
            .iter()
            .filter_map(|v| grid.get_from(origin, *v))
            .cloned()
            .collect::<Vec<char>>();

        if mas == ['M', 'A', 'S'] {
            count += 1;
        }
    }
    (count > 0).then_some(count)
}

pub fn part2(input: &Grid<char>) -> u32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input.width() {
            let origin = Vector::new(i, j);
            if input.get(origin) == Some(&'A') {
                if let Some(count_found) = search_ms(input, origin) {
                    count += count_found;
                }
            }
        }
    }
    count
}

pub fn search_ms(grid: &Grid<char>, origin: Vector) -> Option<u32> {
    let mut count = 0;
    let [left_vector, right_vector] = [[UP_LEFT, DOWN_RIGHT], [UP_RIGHT, DOWN_LEFT]];

    let left = left_vector
        .iter()
        .filter_map(|v| grid.get_from(origin, *v))
        .cloned()
        .collect::<Vec<char>>();

    let right = right_vector
        .iter()
        .filter_map(|v| grid.get_from(origin, *v))
        .cloned()
        .collect::<Vec<char>>();

    if (left == ['M', 'S'] || left == ['S', 'M']) && (right == ['M', 'S'] || right == ['S', 'M']) {
        count += 1;
    }

    (count > 0).then_some(count)
}
