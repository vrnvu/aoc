use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

#[derive(Debug)]
struct Equation {
    result: u64,
    elements: Vec<u64>,
}

impl FromStr for Equation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, elements) = s.split_once(": ").unwrap();
        let result = result.parse()?;
        let elements = elements
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { result, elements })
    }
}

impl Equation {
    fn has_solution_part1(&self) -> bool {
        // state: (index, result)
        let mut stack = VecDeque::new();
        stack.push_back((1, self.elements[0]));
        while let Some((index, result)) = stack.pop_back() {
            if index + 1 > self.elements.len() {
                if result == self.result {
                    return true;
                }
                continue;
            }
            if result > self.result {
                continue;
            }
            stack.push_back((index + 1, result + self.elements[index]));
            stack.push_back((index + 1, result * self.elements[index]));
        }
        false
    }

    fn has_solution_part2(&self) -> bool {
        // state: (index, result)
        let mut stack = VecDeque::new();
        stack.push_back((1, self.elements[0]));
        while let Some((index, result)) = stack.pop_back() {
            if index + 1 > self.elements.len() {
                if result == self.result {
                    return true;
                }
                continue;
            }
            if result > self.result {
                continue;
            }
            stack.push_back((index + 1, result + self.elements[index]));
            stack.push_back((index + 1, result * self.elements[index]));
            stack.push_back((
                index + 1,
                format!("{}{}", result, self.elements[index])
                    .parse()
                    .unwrap(),
            ));
        }
        false
    }

    fn _has_solution_recursive(&self, index: usize, result: u64) -> bool {
        if index + 1 > self.elements.len() {
            return result == self.result;
        }
        let c1 = self._has_solution_recursive(index + 1, result + self.elements[index]);
        let c2 = self._has_solution_recursive(index + 1, result * self.elements[index]);
        let c3 = self._has_solution_recursive(
            index + 1,
            format!("{}{}", result, self.elements[index])
                .parse()
                .unwrap(),
        );
        c1 || c2 || c3
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .par_bridge()
        .map(|line| line.parse::<Equation>().unwrap())
        .filter(Equation::has_solution_part1)
        .map(|equation| equation.result)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .par_bridge()
        .map(|line| line.parse::<Equation>().unwrap())
        .filter(Equation::has_solution_part2)
        .map(|equation| equation.result)
        .sum()
}
