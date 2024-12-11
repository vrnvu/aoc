use aoc::year2015::day05::*;

#[test]
fn part1_test() {
    let input = parse("ugknbfddgicrmopn");
    assert_eq!(part1(&input), 1);

    let input = parse("aaa");
    assert_eq!(part1(&input), 1);

    let input = parse("jchzalrnumimnmhp");
    assert_eq!(part1(&input), 0);

    let input = parse("haegwjzuvuyypxyu");
    assert_eq!(part1(&input), 0);

    let input = parse("dvszwmarrgswjxmb");
    assert_eq!(part1(&input), 0);
}

#[test]
fn part2_test() {
    let input = parse("qjhvhtzxzqqjkmpb");
    assert_eq!(part2(&input), 1);

    let input = parse("xxyxx");
    assert_eq!(part2(&input), 1);

    // "xyx" is a repeat
    // "aaa" is not valid pair
    let input = parse("xyxaaa");
    assert_eq!(part2(&input), 0);

    // "aaaaef" is valid pair
    let input = parse("aaaaef");
    assert_eq!(part2(&input), 1);

    let input = parse("uurcxstgmygtbstg");
    assert_eq!(part2(&input), 0);

    let input = parse("ieodomkazucvgmuy");
    assert_eq!(part2(&input), 0);
}
