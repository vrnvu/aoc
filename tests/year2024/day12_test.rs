use aoc::year2024::day12::*;

const EXAMPLE0: &str = "\
AAAA
BBCD
BBCC
EEEC";

const EXAMPLE1: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const EXAMPLE2: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const EXAMPLE3: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE0);
    assert_eq!(part1(&input), 140);

    let input = parse(EXAMPLE1);
    assert_eq!(part1(&input), 772);

    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1930);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE0);
    assert_eq!(part2(&input), 80);

    let input = parse(EXAMPLE1);
    assert_eq!(part2(&input), 436);

    let input = parse(EXAMPLE2);
    assert_eq!(part2(&input), 236);

    let input = parse(EXAMPLE3);
    assert_eq!(part2(&input), 368);

    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1206);
}
