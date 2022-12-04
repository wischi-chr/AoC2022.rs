use std::{
    env,
    io::{stdin, BufRead},
    mem,
};

use crate::aoc2022::{find_duplicate, Calories, Elf, FingerShape, Food, GameOutcome};

fn main() {
    let day = get_first_arg();

    let mut std_lines = stdin()
        .lock()
        .lines()
        .map(|x| x.expect("Line couldn't be read."));

    let result = match day.as_str() {
        "1a" => day01a(&mut std_lines),
        "1b" => day01b(&mut std_lines),
        "2a" => day02a(&mut std_lines),
        "2b" => day02b(&mut std_lines),
        "3a" => day03a(&mut std_lines),
        x => todo!("day with code '{}' not (yet?) implemented", x),
    };

    println!("The solution for day {} is '{}'", day, result);
}

fn day01a<I>(lines: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    let elves = read_elves_from_food_list(lines);

    elves
        .iter()
        .map(|x| x.get_total_calories())
        .max()
        .expect("Expected at least a single elf.")
        .to_string()
}

fn day01b<I>(lines: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    let mut elves = read_elves_from_food_list(lines);

    // TODO: maybe solve that with a single pass (without sorting all items)
    elves.sort_by(Elf::cmp_calories_desc);

    elves
        .iter()
        .take(3)
        .map(|x| x.get_total_calories())
        .sum::<Calories>()
        .to_string()
}

fn day02a<I>(lines: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    lines
        .map(|x| {
            let raw = x.as_bytes();
            assert!(raw.len() == 3);
            assert!(raw[1] == b' ');

            let opponent = FingerShape::parse_opponent_shape(raw[0]);
            let mine = FingerShape::parse_my_shape_day2a(raw[2]);

            let outcome = mine.play(&opponent);

            let shape_score = match mine {
                FingerShape::Rock => 1,
                FingerShape::Paper => 2,
                FingerShape::Scissors => 3,
            };

            let outcome_score = match outcome {
                GameOutcome::Lose => 0,
                GameOutcome::Draw => 3,
                GameOutcome::Win => 6,
            };

            outcome_score + shape_score
        })
        .sum::<i32>()
        .to_string()
}

fn day02b<I>(lines: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    lines
        .map(|x| {
            let raw = x.as_bytes();
            assert!(raw.len() == 3);
            assert!(raw[1] == b' ');

            let opponent = FingerShape::parse_opponent_shape(raw[0]);
            let outcome = GameOutcome::parse_outcome_day2b(raw[2]);

            let mine = FingerShape::get_my_shape(&opponent, outcome);

            let shape_score = match mine {
                FingerShape::Rock => 1,
                FingerShape::Paper => 2,
                FingerShape::Scissors => 3,
            };

            let outcome_score = match outcome {
                GameOutcome::Lose => 0,
                GameOutcome::Draw => 3,
                GameOutcome::Win => 6,
            };

            outcome_score + shape_score
        })
        .sum::<i32>()
        .to_string()
}

fn day03a<I>(lines: &mut I) -> String
where
    I: Iterator<Item = String>,
{
    lines
        .map(|x| {
            let byte_data = x.as_bytes();
            let len = byte_data.len();

            assert!(len % 2 == 0);
            assert!(len >= 2);

            let half = len / 2;

            let compartment1 = &byte_data[0..half];
            let compartment2 = &byte_data[half..];

            let positions =
                find_duplicate(compartment1, compartment2).expect("No duplicate item found");

            let item = compartment1[positions.0];

            let priority = if item >= b'a' && item <= b'z' {
                item - b'a' + 1
            } else {
                item - b'A' + 27
            };

            priority as i32
        })
        .sum::<i32>()
        .to_string()
}

fn read_elves_from_food_list<I>(lines: &mut I) -> Vec<Elf>
where
    I: Iterator<Item = String>,
{
    let mut elves: Vec<Elf> = vec![];
    let mut current_elf = Elf::new();

    for line in lines {
        if line.is_empty() {
            // finalize current elf and create new one
            let mut elf = Elf::new();
            mem::swap(&mut elf, &mut current_elf);
            elves.push(elf);

            continue;
        }

        let food_calories = line
            .parse::<u32>()
            .expect("Food calories not a valid number.");

        current_elf.add_food(Food::new(food_calories));
    }

    elves.push(current_elf);

    elves
}

fn get_first_arg() -> String {
    env::args()
        .nth(1)
        .expect("Expected to find a cli parameter.")
}

mod aoc2022 {
    use std::cmp::Ordering;

    pub type Calories = u32;

    pub fn find_duplicate<T>(a: &[T], b: &[T]) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for (a_idx, a_item) in a.iter().enumerate() {
            for (b_idx, b_item) in b.iter().enumerate() {
                if a_item.eq(b_item) {
                    return Some((a_idx, b_idx));
                }
            }
        }

        None
    }

    pub struct Food {
        calories: Calories,
    }

    pub struct Elf {
        food: Vec<Food>,
        total_calories: Calories,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FingerShape {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GameOutcome {
        Draw,
        Lose,
        Win,
    }

    impl GameOutcome {
        pub fn parse_outcome_day2b(c: u8) -> Self {
            match c {
                b'X' => Self::Lose,
                b'Y' => Self::Draw,
                b'Z' => Self::Win,
                _ => panic!("Invalid code for outcome: '{}'.", c),
            }
        }
    }

    impl FingerShape {
        pub fn parse_opponent_shape(c: u8) -> Self {
            match c {
                b'A' => Self::Rock,
                b'B' => Self::Paper,
                b'C' => Self::Scissors,
                _ => panic!("Invalid code for opponent shape: '{}'.", c),
            }
        }

        pub fn parse_my_shape_day2a(c: u8) -> Self {
            match c {
                b'X' => Self::Rock,
                b'Y' => Self::Paper,
                b'Z' => Self::Scissors,
                _ => panic!("Invalid code for my shape: '{}'.", c),
            }
        }

        pub fn play(&self, other: &FingerShape) -> GameOutcome {
            match self {
                Self::Paper => match other {
                    Self::Paper => GameOutcome::Draw,
                    Self::Rock => GameOutcome::Win,
                    Self::Scissors => GameOutcome::Lose,
                },
                Self::Rock => match other {
                    Self::Rock => GameOutcome::Draw,
                    Self::Scissors => GameOutcome::Win,
                    Self::Paper => GameOutcome::Lose,
                },
                Self::Scissors => match other {
                    Self::Scissors => GameOutcome::Draw,
                    Self::Paper => GameOutcome::Win,
                    Self::Rock => GameOutcome::Lose,
                },
            }
        }

        pub fn get_my_shape(opponent: &Self, desired_outcome: GameOutcome) -> FingerShape {
            match desired_outcome {
                GameOutcome::Lose => match opponent {
                    FingerShape::Paper => FingerShape::Rock,
                    FingerShape::Rock => FingerShape::Scissors,
                    FingerShape::Scissors => FingerShape::Paper,
                },
                GameOutcome::Win => match opponent {
                    FingerShape::Paper => FingerShape::Scissors,
                    FingerShape::Rock => FingerShape::Paper,
                    FingerShape::Scissors => FingerShape::Rock,
                },
                GameOutcome::Draw => *opponent,
            }
        }
    }

    impl Elf {
        pub fn new() -> Self {
            Elf {
                food: vec![],
                total_calories: 0,
            }
        }

        pub fn add_food(&mut self, food: Food) {
            self.total_calories += food.calories;
            self.food.push(food);
        }

        pub fn get_total_calories(&self) -> Calories {
            self.total_calories
        }

        pub fn cmp_calories_desc(a: &Elf, b: &Elf) -> Ordering {
            b.get_total_calories().cmp(&a.get_total_calories())
        }
    }

    impl Food {
        pub fn new(calories: Calories) -> Self {
            Food { calories }
        }
    }
}
