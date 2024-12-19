use aoc::year2024::day14::*;

const EXAMPLE: &str = "\
11 7
0,4 3,-3
6,3 -1,-3
10,3 -1,2
2,0 2,-1
0,0 1,3
3,0 -2,-2
7,6 -1,-3
3,0 -1,-2
9,3 2,3
7,3 -1,2
2,4 2,-3
9,5 -3,-3";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 12);
}

#[test]
fn part2_test() {}
