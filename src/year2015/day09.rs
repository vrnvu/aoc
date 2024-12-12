use std::collections::{HashMap, HashSet};

type Flight = (String, String, usize);

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Flight> {
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                [from, "to", to, "=", distance] => {
                    (from.to_string(), to.to_string(), distance.parse().unwrap())
                }
                _ => panic!("Invalid line: {}", line),
            },
        )
        .collect()
}

pub fn part1(input: &[Flight]) -> usize {
    find_path_distance(input, true)
}

pub fn part2(input: &[Flight]) -> usize {
    find_path_distance(input, false)
}

fn find_path_distance(input: &[Flight], optimize_shortest: bool) -> usize {
    let mut all_cities = HashSet::new();
    let mut flights: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for (from, to, distance) in input {
        all_cities.insert(from);
        all_cities.insert(to);
        flights
            .entry(from.clone())
            .or_default()
            .push((to.clone(), *distance));
        flights
            .entry(to.clone())
            .or_default()
            .push((from.clone(), *distance));
    }

    let mut best_distance = if optimize_shortest { usize::MAX } else { 0 };
    for &city in all_cities.iter() {
        let mut stack = vec![(HashSet::new(), city, 0)];

        while let Some((mut visited, current_city, distance)) = stack.pop() {
            if !visited.insert(current_city) {
                continue;
            }

            if visited.len() == all_cities.len() {
                best_distance = if optimize_shortest {
                    best_distance.min(distance)
                } else {
                    best_distance.max(distance)
                };
                continue;
            }

            if let Some(destinations) = flights.get(current_city) {
                for (to, to_distance) in destinations {
                    stack.push((visited.clone(), to, distance + to_distance));
                }
            }
        }
    }

    best_distance
}
