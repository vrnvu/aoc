use std::str::FromStr;

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

#[derive(Debug)]
struct Computer {
    ops: Vec<Op>,
}

impl Computer {
    fn load(input: &str) -> Self {
        let re = regex::Regex::new(
            r"(?:(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\)))",
        )
        .unwrap();
        let ops = re
            .captures_iter(input)
            .map(|c| c.get(0).unwrap().as_str().parse::<Op>().unwrap())
            .collect();
        Self { ops }
    }

    fn exec(self) -> u32 {
        let mut result = 0;
        let mut do_mul = true;
        for command in self.ops.iter() {
            match command {
                Op::Do => do_mul = true,
                Op::Dont => do_mul = false,
                Op::Mul(a, b) => {
                    if do_mul {
                        result += a * b;
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
enum Op {
    Mul(u32, u32),
    Do,
    Dont,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mul") {
            let (a, b) = s[4..s.len() - 1].split_once(',').unwrap();
            Ok(Self::Mul(a.parse().unwrap(), b.parse().unwrap()))
        } else if s.starts_with("don't") {
            Ok(Self::Dont)
        } else if s.starts_with("do") {
            Ok(Self::Do)
        } else {
            Err(())
        }
    }
}

pub fn part3(input: &str) -> u32 {
    let computer = Computer::load(input);
    computer.exec()
}

pub fn part2(input: &str) -> u32 {
    let re =
        regex::Regex::new(r"(?:(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\)))")
            .unwrap();

    let mut result = 0;
    let mut do_mul = true;
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            if capture.name("do").is_some() {
                do_mul = true;
            } else if capture.name("dont").is_some() {
                do_mul = false;
            } else if capture.name("mul").is_some() && do_mul {
                let a = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let b = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
                result += a * b;
            }
        }
    }
    result
}

pub fn part1(input: &str) -> u32 {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for line in input.lines() {
        for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
            result += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        }
    }
    result
}
