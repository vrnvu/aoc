pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub struct Input {
    length: u64,
    width: u64,
    height: u64,
}

pub fn parse(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('x');
            let length = parts.next().unwrap().parse().unwrap();
            let width = parts.next().unwrap().parse().unwrap();
            let height = parts.next().unwrap().parse().unwrap();
            Input {
                length,
                width,
                height,
            }
        })
        .collect()
}

pub fn part1(input: &[Input]) -> u64 {
    input
        .iter()
        .map(|i| {
            let surface =
                (2 * i.length * i.width) + (2 * i.width * i.height) + (2 * i.height * i.length);
            let area1 = i.length * i.width;
            let area2 = i.width * i.height;
            let area3 = i.height * i.length;
            surface + area1.min(area2).min(area3)
        })
        .sum()
}

pub fn part2(input: &[Input]) -> u64 {
    input
        .iter()
        .map(|i| {
            let permiter1 = 2 * (i.length + i.width);
            let permiter2 = 2 * (i.width + i.height);
            let permiter3 = 2 * (i.height + i.length);
            let cubic_feet = i.length * i.width * i.height;
            permiter1.min(permiter2).min(permiter3) + cubic_feet
        })
        .sum()
}
