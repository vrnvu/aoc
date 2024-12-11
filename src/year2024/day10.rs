use std::collections::HashSet;

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
    grid
}

fn count_trailheads(grid: &[Vec<u8>], starting_coords: (usize, usize)) -> usize {
    let mut trailheads = 0;
    let mut stack = Vec::new();
    let mut visited = HashSet::with_capacity(grid.len() * grid[0].len());

    stack.push(starting_coords);
    while let Some((i, j)) = stack.pop() {
        if !visited.insert((i, j)) {
            continue;
        }

        if grid[i][j] == 9 {
            trailheads += 1;
            continue;
        }

        let neighbors = [
            (i.checked_sub(1).map(|x| (x, j))),
            (i.checked_add(1).map(|x| (x, j))),
            (j.checked_sub(1).map(|y| (i, y))),
            (j.checked_add(1).map(|y| (i, y))),
        ];
        for next_pos in neighbors.into_iter().flatten() {
            let (ni, nj) = next_pos;
            if ni >= grid.len() || nj >= grid[0].len() {
                continue;
            }
            if grid[ni][nj] != grid[i][j] + 1 {
                continue;
            }

            stack.push((ni, nj));
        }
    }

    trailheads
}

pub fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut trailheads = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                trailheads += count_trailheads(&grid, (i, j));
            }
        }
    }
    trailheads
}

fn count_all_trailheads(grid: &[Vec<u8>], starting_coords: (usize, usize)) -> usize {
    let mut all_trailheads = 0;
    let mut stack = Vec::new();

    stack.push(starting_coords);

    while let Some((i, j)) = stack.pop() {
        if grid[i][j] == 9 {
            all_trailheads += 1;
            continue;
        }

        let neighbors = [
            (i.checked_sub(1).map(|x| (x, j))),
            (i.checked_add(1).map(|x| (x, j))),
            (j.checked_sub(1).map(|y| (i, y))),
            (j.checked_add(1).map(|y| (i, y))),
        ];
        for next_pos in neighbors.into_iter().flatten() {
            let (ni, nj) = next_pos;
            if ni >= grid.len() || nj >= grid[0].len() {
                continue;
            }
            if grid[ni][nj] != grid[i][j] + 1 {
                continue;
            }

            stack.push((ni, nj));
        }
    }

    all_trailheads
}

pub fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut trailheads = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                trailheads += count_all_trailheads(&grid, (i, j));
            }
        }
    }
    trailheads
}
