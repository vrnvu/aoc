use aoc::year2015::day12::*;

#[test]
fn part1_test() {
    let input = parse("[1,2,3]");
    assert_eq!(part1(&input), 6);

    let input = parse("{\"a\":2,\"b\":4}");
    assert_eq!(part1(&input), 6);

    let input = parse("[[[3]]]");
    assert_eq!(part1(&input), 3);

    let input = parse("{\"a\":{\"b\":4},\"c\":-1}");
    assert_eq!(part1(&input), 3);

    let input = parse("{\"a\":[-1,1]}");
    assert_eq!(part1(&input), 0);

    let input = parse("[-1,{\"a\":1}]");
    assert_eq!(part1(&input), 0);

    let input = parse("[]");
    assert_eq!(part1(&input), 0);

    let input = parse("{}");
    assert_eq!(part1(&input), 0);
}

#[test]
fn part2_test() {
    let input = parse("[1,2,3]");
    assert_eq!(part2(&input), 6);

    let input = parse("[1,{\"c\":\"red\",\"b\":2},3]");
    assert_eq!(part2(&input), 4);

    let input = parse("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}");
    assert_eq!(part2(&input), 0);

    let input = parse("[1,\"red\",5]");
    assert_eq!(part2(&input), 6);
}
