use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day2;

impl PuzzleSolver for Day2 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let lines = input
            .normalize_line_breaks()
            .split_lf_line_breaks()
            .drop_lf_eof();

        let mut score = 0;

        for line in lines {
            assert!(line.len() == 3);
            assert!(line[1] == b' ');

            let first_code = line[0];
            let second_code = line[2];

            let opponent_gesture = HandGesture::parse_opponent_gesture(first_code);

            let (mine, outcome) = match part {
                PuzzlePart::Part1 => {
                    let g = HandGesture::parse_my_gesture_part1(second_code);
                    let o = g.play(opponent_gesture);

                    (g, o)
                }
                PuzzlePart::Part2 => {
                    let o = GameOutcome::parse_outcome_part2(second_code);
                    let g = HandGesture::get_my_shape(opponent_gesture, o);

                    (g, o)
                }
            };

            let shape_score = match mine {
                HandGesture::Rock => 1,
                HandGesture::Paper => 2,
                HandGesture::Scissors => 3,
            };

            let outcome_score = match outcome {
                GameOutcome::Lose => 0,
                GameOutcome::Draw => 3,
                GameOutcome::Win => 6,
            };

            score += outcome_score + shape_score
        }

        score.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandGesture {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameOutcome {
    Draw,
    Lose,
    Win,
}

impl HandGesture {
    pub fn parse_opponent_gesture(c: u8) -> Self {
        match c {
            b'A' => Self::Rock,
            b'B' => Self::Paper,
            b'C' => Self::Scissors,
            _ => panic!("Invalid code for opponent shape: '{c}'."),
        }
    }

    pub fn parse_my_gesture_part1(c: u8) -> Self {
        match c {
            b'X' => Self::Rock,
            b'Y' => Self::Paper,
            b'Z' => Self::Scissors,
            _ => panic!("Invalid code for my shape: '{c}'."),
        }
    }

    pub const fn play(self, other: Self) -> GameOutcome {
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

    pub const fn get_my_shape(opponent: Self, desired_outcome: GameOutcome) -> Self {
        match desired_outcome {
            GameOutcome::Lose => match opponent {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            },
            GameOutcome::Win => match opponent {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
            GameOutcome::Draw => opponent,
        }
    }
}

impl GameOutcome {
    pub fn parse_outcome_part2(c: u8) -> Self {
        match c {
            b'X' => Self::Lose,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => panic!("Invalid code for outcome: '{c}'."),
        }
    }
}
