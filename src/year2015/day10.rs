pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part1(input: &[char]) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut result = input.to_vec();
    for _ in 1..=40 {
        result = run_once(&result);
    }

    result.len()
}

pub fn part2(input: &[char]) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut result = input.to_vec();
    for _ in 1..=50 {
        result = run_once(&result);
    }

    result.len()
}

fn run_once(input: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::with_capacity(input.len());
    let mut current_char: char = input[0];
    let mut current_count = 1;

    for &char in &input[1..] {
        if current_char == char {
            current_count += 1;
        } else {
            result.push(char::from_digit(current_count, 10).unwrap());
            result.push(current_char);

            current_char = char;
            current_count = 1;
        }
    }

    result.push(char::from_digit(current_count, 10).unwrap());
    result.push(current_char);
    result
}
