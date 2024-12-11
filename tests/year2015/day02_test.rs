use aoc::year2015::day02::*;

const EXAMPLE1: &str = "2x3x4";
const EXAMPLE2: &str = "1x1x10";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 58);

    let input = parse(EXAMPLE2);
    assert_eq!(part1(&input), 43);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE1);
    assert_eq!(part2(&input), 34);

    let input = parse(EXAMPLE2);
    assert_eq!(part2(&input), 14);
}
