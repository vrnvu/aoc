use aoc::year2015::day01::*;

const EXAMPLE1: &str = "(())";
const EXAMPLE2: &str = "(()(()(";
const EXAMPLE3: &str = "))(((((";
const EXAMPLE4: &str = ")())())";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 0);

    let input = parse(EXAMPLE2);
    assert_eq!(part1(&input), 3);

    let input = parse(EXAMPLE3);
    assert_eq!(part1(&input), 3);

    let input = parse(EXAMPLE4);
    assert_eq!(part1(&input), -3);
}

#[test]
fn part2_test() {
    let input = parse(")");
    assert_eq!(part2(&input), 1);

    let input = parse("()())");
    assert_eq!(part2(&input), 5);
}
