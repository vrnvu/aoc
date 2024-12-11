pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

type Input = (Box<[i64]>, Box<[i64]>);

pub fn parse(input: &str) -> Input {
    to_slices(input)
}

fn to_slices(input: &str) -> (Box<[i64]>, Box<[i64]>) {
    let line_count = input.lines().count();
    let mut lefts = Vec::with_capacity(line_count);
    let mut rights = Vec::with_capacity(line_count);

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse::<i64>().unwrap();
        let right = iter.next().unwrap().parse::<i64>().unwrap();
        lefts.push(left);
        rights.push(right);
    }
    assert_eq!(lefts.len(), rights.len());

    // NOTE: Vec<i64> vs Box<[i64]>
    // https://blog.polybdenum.com/2024/01/17/identifying-the-collect-vec-memory-leak-footgun.html
    (lefts.into_boxed_slice(), rights.into_boxed_slice())
}

pub fn part1(input: &Input) -> i64 {
    let (mut lefts, mut rights) = input.clone();
    lefts.sort();
    rights.sort();

    let mut result = 0;
    for (left, right) in lefts.iter().zip(rights.iter()) {
        result += (left - right).abs();
    }

    result
}

pub fn part2(input: &Input) -> i64 {
    let (lefts, rights) = input.clone();

    let mut result = 0;
    for left in lefts {
        let count = rights.iter().filter(|right| left == **right).count() as i64;
        result += count * left;
    }

    result
}
