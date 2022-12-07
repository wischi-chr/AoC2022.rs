mod aoc_general;
mod common;
mod day01;
mod day02;
mod day03;
mod day04;

use std::{
    env,
    io::{stdin, Read},
};

fn main() {
    let day = get_first_arg();

    let mut std_bytes = stdin()
        .lock()
        .bytes()
        .map(|x| x.expect("reading stdin should not fail"));

    let result = match day.as_str() {
        "1" => day01::solve(&mut std_bytes),
        "2" => day02::solve(&mut std_bytes),
        "3" => day03::solve(&mut std_bytes),
        "4" => day04::solve(&mut std_bytes),
        x => todo!("day with code '{}' not (yet?) implemented", x),
    };

    println!(
        "The solutions for day {} are '{}' and '{}'",
        day, result.0, result.1
    );
}

fn get_first_arg() -> String {
    env::args()
        .nth(1)
        .expect("Expected to find a cli parameter.")
}
