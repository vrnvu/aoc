use regex::Regex;

#[derive(Debug)]
pub enum Action {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Action> {
    let re_turn_on = Regex::new(r"turn on (\d+,\d+) through (\d+,\d+)").unwrap();
    let re_turn_off = Regex::new(r"turn off (\d+,\d+) through (\d+,\d+)").unwrap();
    let re_toggle = Regex::new(r"toggle (\d+,\d+) through (\d+,\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(captures) = re_turn_on.captures(line) {
                let start = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                let end = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                Action::TurnOn((start[0], start[1]), (end[0], end[1]))
            } else if let Some(captures) = re_turn_off.captures(line) {
                let start = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                let end = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                Action::TurnOff((start[0], start[1]), (end[0], end[1]))
            } else if let Some(captures) = re_toggle.captures(line) {
                let start = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                let end = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                Action::Toggle((start[0], start[1]), (end[0], end[1]))
            } else {
                panic!("Invalid input: {}", line);
            }
        })
        .collect()
}

pub fn part1(input: &[Action]) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    for action in input {
        match action {
            Action::TurnOn(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i][j] = true;
                    }
                }
            }
            Action::TurnOff(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i][j] = false;
                    }
                }
            }
            Action::Toggle(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i][j] = !grid[i][j];
                    }
                }
            }
        }
    }

    let mut count = 0;
    for row in grid {
        for cell in row {
            if cell {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &[Action]) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    for action in input {
        match action {
            Action::TurnOn(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i][j] += 1;
                    }
                }
            }
            Action::TurnOff(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        if grid[i][j] > 0 {
                            grid[i][j] -= 1;
                        }
                    }
                }
            }
            Action::Toggle(start, end) => {
                for i in start.0..=end.0 {
                    for j in start.1..=end.1 {
                        grid[i][j] += 2;
                    }
                }
            }
        }
    }

    let mut brightness = 0;
    for row in grid {
        for cell in row {
            brightness += cell;
        }
    }
    brightness
}
