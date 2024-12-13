use crate::utils::grid::Vector;

#[derive(Debug)]
pub struct Equation {
    a1: isize,
    a2: isize,
    b1: isize,
    b2: isize,
    c1: isize,
    c2: isize,
}

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Equation> {
    let mut lines = input.lines();
    let mut equations = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut button_a = line.split(',').map(|s| s.parse::<isize>().unwrap());

        let line = lines.next().unwrap();
        let mut button_b = line.split(',').map(|s| s.parse::<isize>().unwrap());

        let line = lines.next().unwrap();
        let mut prize = line.split(',').map(|s| s.parse::<isize>().unwrap());

        equations.push(Equation {
            a1: button_a.next().unwrap(),
            a2: button_a.next().unwrap(),
            b1: button_b.next().unwrap(),
            b2: button_b.next().unwrap(),
            c1: prize.next().unwrap(),
            c2: prize.next().unwrap(),
        });
    }
    equations
}

pub fn part1(input: &[Equation]) -> isize {
    input
        .iter()
        .filter_map(|equation| solve_equation(equation, Some(100)))
        .map(|(step_a, step_b)| step_a * 3 + step_b)
        .sum()
}

pub fn part2(input: &[Equation]) -> isize {
    input
        .iter()
        .map(|equation| Equation {
            a1: equation.a1,
            a2: equation.a2,
            b1: equation.b1,
            b2: equation.b2,
            c1: equation.c1 + 10000000000000,
            c2: equation.c2 + 10000000000000,
        })
        .filter_map(|equation| solve_equation(&equation, None))
        .map(|(step_a, step_b)| step_a * 3 + step_b)
        .sum()
}

fn solve_equation(equation: &Equation, max_step: Option<isize>) -> Option<(isize, isize)> {
    let denominator = (-equation.a2 * equation.b1) + (equation.b2 * equation.a1);
    if denominator == 0 {
        return None;
    }

    let step_b = (equation.c2 * equation.a1 - equation.c1 * equation.a2) / denominator;
    let step_a = (equation.c1 - equation.b1 * step_b) / equation.a1;

    if step_a < 0
        || step_b < 0
        || step_a >= max_step.unwrap_or(isize::MAX)
        || step_b >= max_step.unwrap_or(isize::MAX)
    {
        return None;
    }

    if step_a * equation.a1 + step_b * equation.b1 != equation.c1
        || step_a * equation.a2 + step_b * equation.b2 != equation.c2
    {
        return None;
    }

    Some((step_a, step_b))
}
