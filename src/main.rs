use std::{env::args, str::FromStr};

use aoc::{year2015, year2024};

enum Year {
    Year2024,
    Year2023,
    Year2022,
    Year2021,
    Year2020,
    Year2019,
    Year2018,
    Year2017,
    Year2016,
    Year2015,
}

impl FromStr for Year {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2024" => Year::Year2024,
            "2023" => Year::Year2023,
            "2022" => Year::Year2022,
            "2021" => Year::Year2021,
            "2020" => Year::Year2020,
            "2019" => Year::Year2019,
            "2018" => Year::Year2018,
            "2017" => Year::Year2017,
            "2016" => Year::Year2016,
            "2015" => Year::Year2015,
            _ => return Err(()),
        })
    }
}

enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl FromStr for Day {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "01" => Day::Day01,
            "02" => Day::Day02,
            "03" => Day::Day03,
            "04" => Day::Day04,
            "05" => Day::Day05,
            "06" => Day::Day06,
            "07" => Day::Day07,
            "08" => Day::Day08,
            "09" => Day::Day09,
            "10" => Day::Day10,
            "11" => Day::Day11,
            "12" => Day::Day12,
            "13" => Day::Day13,
            "14" => Day::Day14,
            "15" => Day::Day15,
            "16" => Day::Day16,
            "17" => Day::Day17,
            "18" => Day::Day18,
            "19" => Day::Day19,
            "20" => Day::Day20,
            "21" => Day::Day21,
            "22" => Day::Day22,
            "23" => Day::Day23,
            "24" => Day::Day24,
            "25" => Day::Day25,
            _ => return Err(()),
        })
    }
}

fn main() {
    let (year, day) = match args().collect::<Vec<String>>().as_slice() {
        [_, year, day] => (year.to_string(), day.to_string()),
        _ => panic!("Usage: aoc <year> <day>"),
    };
    let data = fs_err::read_to_string(format!("input/year{}/day{}.txt", year, day)).unwrap();
    let (part1, part2) = run_solution(&year, &day, &data);

    println!("{}", part1);
    println!("{}", part2);
}

fn run_solution(year: &str, day: &str, data: &str) -> (String, String) {
    let (year, day) = (year.parse().unwrap(), day.parse().unwrap());
    match (year, day) {
        (Year::Year2015, Day::Day01) => year2015::day01::run(data),
        (Year::Year2015, Day::Day02) => year2015::day02::run(data),
        (Year::Year2015, Day::Day03) => year2015::day03::run(data),
        (Year::Year2015, Day::Day04) => year2015::day04::run(data),
        (Year::Year2015, Day::Day05) => year2015::day05::run(data),
        (Year::Year2024, Day::Day01) => year2024::day01::run(data),
        (Year::Year2024, Day::Day02) => year2024::day02::run(data),
        (Year::Year2024, Day::Day03) => year2024::day03::run(data),
        (Year::Year2024, Day::Day04) => year2024::day04::run(data),
        (Year::Year2024, Day::Day05) => year2024::day05::run(data),
        (Year::Year2024, Day::Day06) => year2024::day06::run(data),
        (Year::Year2024, Day::Day07) => year2024::day07::run(data),
        (Year::Year2024, Day::Day08) => year2024::day08::run(data),
        (Year::Year2024, Day::Day09) => year2024::day09::run(data),
        (Year::Year2024, Day::Day10) => year2024::day10::run(data),
        (Year::Year2024, Day::Day11) => year2024::day11::run(data),
        _ => unimplemented!(),
    }
}
