use std::collections::HashMap;

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

fn digits(stone: u64) -> usize {
    let mut n = stone;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn process_stone(memo: &mut HashMap<(u64, usize), usize>, stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }

    if memo.contains_key(&(stone, blinks)) {
        return memo[&(stone, blinks)];
    }

    if stone == 0 {
        let result = process_stone(memo, 1, blinks - 1);
        memo.insert((stone, blinks), result);
        return result;
    }

    if digits(stone) % 2 == 0 {
        let stone_str = stone.to_string();
        let (left_str, right_str) = stone_str.split_at(digits(stone) / 2);
        let left = left_str.parse::<u64>().unwrap();
        let right = right_str.parse::<u64>().unwrap();

        let result_left = process_stone(memo, left, blinks - 1);
        memo.insert((left, blinks - 1), result_left);

        let result_right = process_stone(memo, right, blinks - 1);
        memo.insert((right, blinks - 1), result_right);

        return result_left + result_right;
    }

    let result = process_stone(memo, stone * 2024, blinks - 1);
    memo.insert((stone, blinks), result);
    return result;
}

pub fn part1(input: &str) -> usize {
    let mut memo = HashMap::new();
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|stone| process_stone(&mut memo, stone, 25))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut memo = HashMap::new();
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|stone| process_stone(&mut memo, stone, 75))
        .sum()
}
