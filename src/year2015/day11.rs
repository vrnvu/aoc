pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<u8> {
    input.bytes().collect()
}

pub fn part1(input: &[u8]) -> String {
    let mut result = input.to_vec();
    while !is_valid(&result) {
        result = increment(&result);
    }
    result.into_iter().map(|c| c as char).collect()
}

pub fn part2(input: &[u8]) -> String {
    let mut result = increment(&parse(&part1(input)));
    while !is_valid(&result) {
        result = increment(&result);
    }
    result.into_iter().map(|c| c as char).collect()
}

fn is_valid(input: &[u8]) -> bool {
    let mut has_straight = false;
    let mut number_of_pairs = 0;
    let mut last_pair_pos = None;

    for (i, &byte) in input.iter().enumerate() {
        if byte == b'i' || byte == b'o' || byte == b'l' {
            return false;
        }
        if i + 2 < input.len() && input[i + 1] == byte + 1 && input[i + 2] == byte + 2 {
            has_straight = true;
        }
        if i + 1 < input.len() && byte == input[i + 1] && last_pair_pos.map_or(true, |pos| pos < i) {
            number_of_pairs += 1;
            last_pair_pos = Some(i + 1);
        }
    }

    has_straight && number_of_pairs >= 2
}

fn increment(input: &[u8]) -> Vec<u8> {
    let mut result = input.to_vec();
    let mut i = result.len() - 1;

    loop {
        if result[i] == b'z' {
            result[i] = b'a';
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            result[i] += 1;
            break;
        }
    }

    result
}
