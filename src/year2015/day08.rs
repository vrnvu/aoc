pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn part1(input: &[Vec<u8>]) -> usize {
    let mut result = 0;
    for line in input {
        result += line.len();
        let mut index = 0;
        let mut chars_len = 0;
        while index < line.len() {
            if line[index] == b'"' {
                index += 1;
            } else if line[index] == b'\\' && line[index + 1] == b'x' {
                chars_len += 1;
                index += 4;
            } else if line[index] == b'\\' {
                chars_len += 1;
                index += 2;
            } else {
                chars_len += 1;
                index += 1;
            }
        }
        result -= chars_len;
    }
    result
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut original_len = 0;
    let mut new_len = 0;
    for line in input {
        original_len += line.len();

        let mut index = 0;
        let mut chars_len = 0;
        while index < line.len() {
            if index == 0 || index == line.len() - 1 {
                chars_len += 3;
                index += 1;
            } else if line[index] == b'"' || line[index] == b'\\' {
                chars_len += 2;
                index += 1;
            } else {
                chars_len += 1;
                index += 1;
            }
        }
        new_len += chars_len;
    }

    new_len - original_len
}
