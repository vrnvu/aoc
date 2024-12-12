use aoc::year2015::day10::*;

#[test]
fn part1_test() {
    let input = parse("1");
    assert_eq!(part1(&input), 82350);
}

#[test]
fn part2_test() {
    let input = parse("1");
    assert_eq!(part2(&input), 1166642);
}
