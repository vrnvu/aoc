use aoc::year2015::day11::*;

#[test]
fn part1_test() {
    let input = parse("abcdefgh");
    assert_eq!(part1(&input), "abcdffaa");

    let input = parse("ghijklmn");
    assert_eq!(part1(&input), "ghjaabcc");
}

#[test]
fn part2_test() {}
