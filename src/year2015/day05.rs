use std::collections::HashMap;

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .filter(|line| {
            let mut vowels = 0;
            let mut double = false;

            for (i, c) in line.chars().enumerate() {
                if "aeiou".contains(c) {
                    vowels += 1;
                }

                if let Some(next_char) = line.chars().nth(i + 1) {
                    if c == next_char {
                        double = true;
                    }
                    if c == 'a' && next_char == 'b' {
                        return false;
                    }
                    if c == 'c' && next_char == 'd' {
                        return false;
                    }
                    if c == 'p' && next_char == 'q' {
                        return false;
                    }
                    if c == 'x' && next_char == 'y' {
                        return false;
                    }
                }
            }
            vowels >= 3 && double
        })
        .count()
}

pub fn part2(input: &[String]) -> usize {
    input
        .iter()
        .filter(|line| {
            let mut pairs = HashMap::new();
            let mut pairs_repeat = false;
            let mut repeats = false;
            for (i, c) in line.chars().enumerate() {
                if i > 0 {
                    match (line.chars().nth(i - 1), line.chars().nth(i + 1)) {
                        (Some(prev), Some(next)) if prev == next => repeats = true,
                        _ => (),
                    }
                }
                if let Some(next) = line.chars().nth(i + 1) {
                    if let Some(last_index) = pairs.get(&(c, next)) {
                        if *last_index != i {
                            pairs_repeat = true;
                        }
                    } else {
                        pairs.insert((c, next), i + 1);
                    }
                }
            }
            pairs_repeat && repeats
        })
        .count()
}
