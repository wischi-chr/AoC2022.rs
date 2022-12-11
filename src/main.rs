mod aoc_general;
mod common;
mod year_2022;

use std::{
    env,
    io::{stdin, Read},
};

use crate::{
    aoc_general::{PuzzlePart, YearSolverCollection},
    year_2022::{
        day01::Day1, day02::Day2, day03::Day3, day04::Day4, day05::Day5, day06::Day6, day07::Day7,
        day08::Day8,
    },
};

fn main() {
    let day = get_first_arg();

    let all_std_bytes = stdin()
        .lock()
        .bytes()
        .map(|x| x.expect("reading stdin should not fail"))
        .collect::<Vec<_>>();

    let mut y2022 = YearSolverCollection::new();
    y2022.add::<Day1>();
    y2022.add::<Day2>();
    y2022.add::<Day3>();
    y2022.add::<Day4>();
    y2022.add::<Day5>();
    y2022.add::<Day6>();
    y2022.add::<Day7>();
    y2022.add::<Day8>();

    let result = match day.as_str() {
        "1a" => (1, PuzzlePart::Part1),
        "1b" => (1, PuzzlePart::Part2),
        "2a" => (2, PuzzlePart::Part1),
        "2b" => (2, PuzzlePart::Part2),
        "3a" => (3, PuzzlePart::Part1),
        "3b" => (3, PuzzlePart::Part2),
        "4a" => (4, PuzzlePart::Part1),
        "4b" => (4, PuzzlePart::Part2),
        "5a" => (5, PuzzlePart::Part1),
        "5b" => (5, PuzzlePart::Part2),
        "6a" => (6, PuzzlePart::Part1),
        "6b" => (6, PuzzlePart::Part2),
        "7a" => (7, PuzzlePart::Part1),
        "7b" => (7, PuzzlePart::Part2),
        "8a" => (8, PuzzlePart::Part1),
        "8b" => (8, PuzzlePart::Part2),
        x => todo!("day with code '{}' not (yet?) implemented", x),
    };

    let mut input = all_std_bytes.iter().copied();
    let solution = y2022.solve(result.0, &mut input, result.1);

    println!("The solution is: '{}'", solution);
}

fn get_first_arg() -> String {
    env::args()
        .nth(1)
        .expect("Expected to find a cli parameter.")
}
