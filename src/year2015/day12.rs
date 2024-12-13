use regex::Regex;

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse::<isize>().unwrap())
        .sum::<isize>() as usize
}

pub fn part2(input: &str) -> isize {
    let parsed_json = json::parse(input).unwrap();
    sum_json_value(&parsed_json)
}

fn sum_json_value(value: &json::JsonValue) -> isize {
    match value {
        json::JsonValue::Number(number) => number.to_string().parse::<isize>().unwrap(),
        json::JsonValue::Object(object) => sum_object(object),
        json::JsonValue::Array(array) => sum_array(array),
        _ => 0,
    }
}

fn sum_object(object: &json::object::Object) -> isize {
    for (_, v) in object.iter() {
        if has_red(v) {
            return 0;
        }
    }
    object.iter().map(|(_, v)| sum_json_value(v)).sum()
}

fn sum_array(array: &[json::JsonValue]) -> isize {
    array.iter().map(sum_json_value).sum()
}

fn has_red(value: &json::JsonValue) -> bool {
    match value {
        json::JsonValue::String(s) => s == "red",
        json::JsonValue::Short(s) => s == "red",
        _ => false,
    }
}
