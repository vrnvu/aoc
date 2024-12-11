use aoc::year2015::day03::*;

const EXAMPLE1: &str = ">";
const EXAMPLE2: &str = "^>v<";
const EXAMPLE3: &str = "^v^v^v^v^v";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 2);

    let input = parse(EXAMPLE2);
    assert_eq!(part1(&input), 4);

    let input = parse(EXAMPLE3);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse("^v");
    assert_eq!(part2(&input), 3);

    let input = parse(EXAMPLE2);
    assert_eq!(part2(&input), 3);

    let input = parse(EXAMPLE3);
    assert_eq!(part2(&input), 11);
}
