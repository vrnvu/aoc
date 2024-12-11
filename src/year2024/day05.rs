use std::collections::HashMap;

pub fn run(input: &str) -> (String, String) {
    (part1(input).to_string(), part2(input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> i32 {
    let mut pages_order: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut result = 0;
    let mut should_build_pages_order = true;
    for line in input.lines() {
        if line.is_empty() {
            should_build_pages_order = false;
            continue;
        }

        if should_build_pages_order {
            build_pages_order(&mut pages_order, line);
        } else if let Some(middle) = validate_pages_order(
            &pages_order,
            &line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        ) {
            result += middle;
        }
    }

    result
}

pub fn build_pages_order(map: &mut HashMap<i32, Vec<i32>>, line: &str) {
    if let Some((previous, next)) = line.split_once('|') {
        if let (Ok(previous), Ok(next)) = (previous.parse::<i32>(), next.parse::<i32>()) {
            map.entry(previous).or_default().push(next);
        }
    }
}

pub fn validate_pages_order(pages_order: &HashMap<i32, Vec<i32>>, pages: &[i32]) -> Option<i32> {
    for i in 0..pages.len() {
        let previous = pages[i];
        for &posterior in pages.iter().skip(i + 1) {
            if let Some(values) = pages_order.get(&posterior) {
                if values.contains(&previous) {
                    return None;
                }
            }
        }
    }
    Some(pages[pages.len() / 2])
}

pub fn part2(input: &str) -> i32 {
    let mut pages_order: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut result = 0;
    let mut should_build_pages_order = true;
    for line in input.lines() {
        if line.is_empty() {
            should_build_pages_order = false;
            continue;
        }

        if should_build_pages_order {
            build_pages_order(&mut pages_order, line);
        } else {
            let mut pages = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if validate_pages_order(&pages_order, &pages).is_none() {
                let middle = fix_page_order(&pages_order, &mut pages);
                result += middle;
            }
        }
    }

    result
}

fn fix_page_order(pages_order: &HashMap<i32, Vec<i32>>, pages: &mut [i32]) -> i32 {
    pages.sort_by(|previous, posterior| {
        if let Some(values) = pages_order.get(posterior) {
            if values.contains(previous) {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Greater
    });
    pages[pages.len() / 2]
}
