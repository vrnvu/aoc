#[derive(Debug, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn new(previous: i64, current: i64) -> anyhow::Result<Self> {
        if is_increasing(previous, current) {
            Ok(Direction::Increasing)
        } else if is_decreasing(previous, current) {
            Ok(Direction::Decreasing)
        } else {
            anyhow::bail!("Invalid direction, {previous} {current}");
        }
    }

    fn is_safe(&self, previous: i64, current: i64) -> bool {
        match self {
            Direction::Increasing => is_increasing(previous, current),
            Direction::Decreasing => is_decreasing(previous, current),
        }
    }
}

fn is_increasing(previous: i64, current: i64) -> bool {
    let diff = current - previous;
    (1..=3).contains(&diff)
}

fn is_decreasing(previous: i64, current: i64) -> bool {
    let diff = previous - current;
    (1..=3).contains(&diff)
}

fn is_safe(row: &[i64]) -> bool {
    let mut direction: Option<Direction> = None;
    let mut previous: Option<i64> = None;

    for &current in row {
        if let Some(p) = previous {
            if let Some(d) = direction {
                if !d.is_safe(p, current) {
                    return false;
                }
            } else {
                direction = match Direction::new(p, current) {
                    Ok(d) => Some(d),
                    Err(_) => return false,
                };
            }
        }
        previous = Some(current);
    }
    true
}

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> i64 {
    let mut safe = 0;
    for line in input.lines() {
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if is_safe(&row[0..]) {
            safe += 1;
        }
    }
    safe
}

fn is_safe_with_skip(row: &[i64], skip_index: Option<usize>) -> bool {
    let mut direction: Option<Direction> = None;
    let mut previous: Option<i64> = None;

    for (i, &current) in row.iter().enumerate() {
        if Some(i) == skip_index {
            continue;
        }
        if let Some(p) = previous {
            if let Some(d) = direction {
                if !d.is_safe(p, current) {
                    return false;
                }
            } else {
                direction = match Direction::new(p, current) {
                    Ok(d) => Some(d),
                    Err(_) => return false,
                };
            }
        }
        previous = Some(current);
    }
    true
}

fn count_violations_with_indices(row: &[i64]) -> (i32, Vec<usize>) {
    let mut direction: Option<Direction> = None;
    let mut previous: Option<i64> = None;
    let mut violations = 0;
    let mut violation_indices = Vec::new();

    for (i, &current) in row.iter().enumerate() {
        if let Some(p) = previous {
            let has_violation = if let Some(d) = direction {
                !d.is_safe(p, current)
            } else {
                Direction::new(p, current).is_err()
            };

            if has_violation {
                violations += 1;
                // Include surrounding indices that might fix the violation
                if i > 1 {
                    violation_indices.push(i - 2);
                }
                violation_indices.push(i - 1);
                violation_indices.push(i);
                if i + 1 < row.len() {
                    violation_indices.push(i + 1);
                }
            }
            direction = Direction::new(p, current).ok();
        }
        previous = Some(current);
    }

    violation_indices.sort_unstable();
    violation_indices.dedup();
    (violations, violation_indices)
}

pub fn part2(input: &str) -> i64 {
    let mut safe = 0;
    for line in input.lines() {
        let row: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let (violations, indices) = count_violations_with_indices(&row);

        if violations == 0 {
            safe += 1;
        } else if violations <= 2 {
            // Try removing elements at violation points
            for &i in &indices {
                if is_safe_with_skip(&row, Some(i)) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}
