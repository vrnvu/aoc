use aoc::year2015::day04::*;

#[test]
fn part1_test() {
    let input = parse("abcdef");
    assert_eq!(part1(&input), 609043);

    let input = parse("pqrstuv");
    assert_eq!(part1(&input), 1048970);
}

#[test]
fn part2_test() {}
