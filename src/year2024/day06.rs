use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

#[derive(Debug, Clone, Default)]
struct Walls {
    row: HashMap<usize, Vec<usize>>,
    col: HashMap<usize, Vec<usize>>,
}

impl Walls {
    fn clone_with_new_wall(&self, (row, col): (usize, usize)) -> Self {
        let mut new_walls = self.clone();
        new_walls.add_wall((row, col));
        new_walls
    }

    fn add_wall(&mut self, (row, col): (usize, usize)) {
        let row_walls = self.row.entry(row).or_default();
        row_walls.push(col);
        row_walls.sort();
        row_walls.dedup();

        let col_walls = self.col.entry(col).or_default();
        col_walls.push(row);
        col_walls.sort();
        col_walls.dedup();
    }

    fn iterate_row(&self, row: usize) -> Vec<usize> {
        self.row.get(&row).unwrap_or(&vec![]).clone()
    }

    fn iterate_col(&self, col: usize) -> Vec<usize> {
        self.col.get(&col).unwrap_or(&vec![]).clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type NextDirection = Option<(Direction, (usize, usize))>;
type Path = Vec<(usize, usize)>;
type NextResult = (NextDirection, Path);

impl Direction {
    fn next_with_path(&self, row: usize, col: usize, walls: &Walls) -> NextResult {
        match self {
            Direction::Up => {
                let walls = walls.iterate_col(col);
                for wall in walls.iter().rev() {
                    if *wall < row {
                        let positions: Vec<(usize, usize)> =
                            (*wall + 1..=row).map(|r| (r, col)).collect();
                        return (Some((Direction::Right, (*wall + 1, col))), positions);
                    }
                }
                let positions: Vec<(usize, usize)> = (0..=row).map(|r| (r, col)).collect();
                (None, positions)
            }
            Direction::Right => {
                let walls = walls.iterate_row(row);
                for wall in walls {
                    if wall > col {
                        let positions: Vec<(usize, usize)> =
                            ((col + 1)..=wall - 1).map(|c| (row, c)).collect();
                        return (Some((Direction::Down, (row, wall - 1))), positions);
                    }
                }
                let positions: Vec<(usize, usize)> = ((col + 1)..=9).map(|c| (row, c)).collect();
                (None, positions)
            }
            Direction::Down => {
                let walls = walls.iterate_col(col);
                for wall in walls {
                    if wall > row {
                        let positions: Vec<(usize, usize)> =
                            ((row + 1)..=wall - 1).map(|r| (r, col)).collect();
                        return (Some((Direction::Left, (wall - 1, col))), positions);
                    }
                }
                let positions: Vec<(usize, usize)> = ((row + 1)..=9).map(|r| (r, col)).collect();
                (None, positions)
            }
            Direction::Left => {
                let walls = walls.iterate_row(row);
                for wall in walls.iter().rev() {
                    if *wall < col {
                        let positions: Vec<(usize, usize)> =
                            (*wall + 1..=col).map(|c| (row, c)).collect();
                        return (Some((Direction::Up, (row, *wall + 1))), positions);
                    }
                }
                let positions: Vec<(usize, usize)> = (0..=col).map(|c| (row, c)).collect();
                (None, positions)
            }
        }
    }

    fn next(
        &self,
        row: usize,
        col: usize,
        walls_row: &[usize],
        walls_col: &[usize],
    ) -> Option<(Direction, usize, usize)> {
        match self {
            Direction::Right => {
                // From left to right
                for &wall in walls_row {
                    if wall > col {
                        return Some((Direction::Down, row, wall - 1));
                    }
                }
                None
            }
            Direction::Left => {
                // From right to left
                for &wall in walls_row.iter().rev() {
                    if wall < col {
                        return Some((Direction::Up, row, wall + 1));
                    }
                }
                None
            }
            Direction::Up => {
                // From bottom to top
                for &wall in walls_col.iter().rev() {
                    if wall < row {
                        return Some((Direction::Right, wall + 1, col));
                    }
                }
                None
            }
            Direction::Down => {
                // From top to bottom
                for &wall in walls_col {
                    if wall > row {
                        return Some((Direction::Left, wall - 1, col));
                    }
                }
                None
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (walls, guard) = build_walls_and_guard(input);
    let visited = move_guard(guard, &walls);
    visited.len()
}

fn build_walls_and_guard(input: &str) -> (Walls, (Direction, usize, usize)) {
    let mut walls = Walls::default();
    let mut guard = None;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                walls.add_wall((row, col));
            } else if c == '^' {
                guard = Some((Direction::Up, row, col));
            }
        }
    }
    (walls, guard.expect("guard not found"))
}

fn move_guard(guard: (Direction, usize, usize), walls: &Walls) -> HashSet<(usize, usize)> {
    let (mut direction, mut row, mut col) = guard;
    let mut visited = vec![(row, col)];
    loop {
        let (next_direction, positions) = direction.next_with_path(row, col, walls);
        visited.extend(positions);

        match next_direction {
            Some((new_dir, (new_row, new_col))) => {
                direction = new_dir;
                row = new_row;
                col = new_col;
            }
            None => break,
        }
    }
    HashSet::from_iter(visited)
}

pub fn part2(input: &str) -> usize {
    let (walls, guard) = build_walls_and_guard(input);
    let visited = move_guard(guard, &walls);
    find_loops(guard, visited, &walls)
}

fn find_loops(
    guard: (Direction, usize, usize),
    visited: HashSet<(usize, usize)>,
    walls: &Walls,
) -> usize {
    let mut loops = 0;
    for (visited_row, visited_col) in visited {
        let new_walls = walls.clone_with_new_wall((visited_row, visited_col));
        if has_loop_with_new_wall(guard, new_walls.clone()) {
            loops += 1;
        }
    }
    loops
}

fn _find_loops_parallel(
    guard: (Direction, usize, usize),
    visited: HashSet<(usize, usize)>,
    walls: &Walls,
) -> usize {
    let loops = Arc::new(AtomicUsize::new(0));
    let mut handlers = vec![];
    for (visited_row, visited_col) in visited {
        let new_walls = walls.clone_with_new_wall((visited_row, visited_col));
        let loops = Arc::clone(&loops);
        let handler = std::thread::spawn(move || {
            if has_loop_with_new_wall(guard, new_walls.clone()) {
                loops.fetch_add(1, Ordering::Relaxed);
            }
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    loops.load(Ordering::Relaxed)
}

fn has_loop_with_new_wall(guard: (Direction, usize, usize), walls: Walls) -> bool {
    let (mut direction, mut row, mut col) = guard;
    let mut visited_positions = HashSet::new();
    visited_positions.insert((row, col, direction));

    while let Some((new_direction, new_row, new_col)) =
        direction.next(row, col, &walls.iterate_row(row), &walls.iterate_col(col))
    {
        if visited_positions.contains(&(new_row, new_col, new_direction)) {
            return true;
        }

        visited_positions.insert((new_row, new_col, new_direction));
        direction = new_direction;
        row = new_row;
        col = new_col;
    }

    false
}
