pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<i64> {
    input
        .bytes()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

pub fn part2(input: &[i64]) -> i64 {
    let mut floor = 0;
    for (i, &d) in input.iter().enumerate() {
        floor += d;
        if floor == -1 {
            return i as i64 + 1;
        }
    }
    unreachable!()
}
