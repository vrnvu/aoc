pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                if let Some(count_found) = search_mas(&grid, i, j) {
                    count += count_found;
                }
            }
        }
    }
    count
}

fn search_mas(grid: &[Vec<char>], i: usize, j: usize) -> Option<u32> {
    let mut count = 0;
    let vectors = vectors(i as isize, j as isize);
    for vector in vectors {
        let mas = vector
            .iter()
            .filter(|(i, j)| *i >= 0 && *j >= 0)
            .filter_map(|(i, j)| {
                grid.get(*i as usize)
                    .and_then(|row| row.get(*j as usize))
                    .copied()
            })
            .collect::<Vec<char>>();

        if mas == ['M', 'A', 'S'] {
            count += 1;
        }
    }
    (count > 0).then_some(count)
}

pub fn vectors(i: isize, j: isize) -> [[(isize, isize); 3]; 8] {
    [
        [(i - 1, j), (i - 2, j), (i - 3, j)],
        [(i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
        [(i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
        [(i, j + 1), (i, j + 2), (i, j + 3)],
        [(i, j - 1), (i, j - 2), (i, j - 3)],
        [(i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
        [(i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
        [(i + 1, j), (i + 2, j), (i + 3, j)],
    ]
}

pub fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                if let Some(count_found) = search_ms(&grid, i, j) {
                    count += count_found;
                }
            }
        }
    }
    count
}

pub fn search_ms(grid: &[Vec<char>], i: usize, j: usize) -> Option<u32> {
    let mut count = 0;
    let [left_vector, right_vector] = vectors_ms(i as isize, j as isize);
    let left = left_vector
        .iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0)
        .filter_map(|(i, j)| {
            grid.get(*i as usize)
                .and_then(|row| row.get(*j as usize))
                .copied()
        })
        .collect::<Vec<char>>();

    let right = right_vector
        .iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0)
        .filter_map(|(i, j)| {
            grid.get(*i as usize)
                .and_then(|row| row.get(*j as usize))
                .copied()
        })
        .collect::<Vec<char>>();

    if (left == ['M', 'S'] || left == ['S', 'M']) && (right == ['M', 'S'] || right == ['S', 'M']) {
        count += 1;
    }

    (count > 0).then_some(count)
}

pub fn vectors_ms(i: isize, j: isize) -> [[(isize, isize); 2]; 2] {
    [
        [(i - 1, j - 1), (i + 1, j + 1)], // up-left + down-right
        [(i - 1, j + 1), (i + 1, j - 1)], // up-right + down-left
    ]
}
