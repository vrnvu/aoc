use aoc::year2024::day13::*;

const EXAMPLE: &str = "\
94,34
22,67
8400,5400

26,66
67,21
12748,12176

17,86
84,37
7870,6450

69,23
27,71
18641,1027";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 480);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 459236326669);
}
