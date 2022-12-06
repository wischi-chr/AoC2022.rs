use crate::{
    aoc_general::PuzzlePart,
    common::{LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

pub fn solve<I>(input: &mut I) -> (String, String)
where
    I: Iterator<Item = u8>,
{
    let lines = input
        .normalize_line_breaks()
        .split_lf_line_breaks()
        .drop_lf_eof();

    let mut score_part1 = 0;
    let mut score_part2 = 0;

    for line in lines {
        assert!(line.len() == 3);
        assert!(line[1] == b' ');

        let opponent_gesture = HandGesture::parse_opponent_gesture(line[0]);

        score_part1 += get_score(&opponent_gesture, line[2], PuzzlePart::Part1);
        score_part2 += get_score(&opponent_gesture, line[2], PuzzlePart::Part2);
    }

    (score_part1.to_string(), score_part2.to_string())
}

fn get_score(opponent_gesture: &HandGesture, my_code: u8, part: PuzzlePart) -> i32 {
    let (mine, outcome) = match part {
        PuzzlePart::Part1 => {
            let g = HandGesture::parse_my_gesture_part1(my_code);
            let o = g.play(opponent_gesture);

            (g, o)
        }
        PuzzlePart::Part2 => {
            let o = GameOutcome::parse_outcome_part2(my_code);
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

    outcome_score + shape_score
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
            _ => panic!("Invalid code for opponent shape: '{}'.", c),
        }
    }

    pub fn parse_my_gesture_part1(c: u8) -> Self {
        match c {
            b'X' => Self::Rock,
            b'Y' => Self::Paper,
            b'Z' => Self::Scissors,
            _ => panic!("Invalid code for my shape: '{}'.", c),
        }
    }

    pub fn play(&self, other: &HandGesture) -> GameOutcome {
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

    pub fn get_my_shape(opponent: &Self, desired_outcome: GameOutcome) -> HandGesture {
        match desired_outcome {
            GameOutcome::Lose => match opponent {
                HandGesture::Paper => HandGesture::Rock,
                HandGesture::Rock => HandGesture::Scissors,
                HandGesture::Scissors => HandGesture::Paper,
            },
            GameOutcome::Win => match opponent {
                HandGesture::Paper => HandGesture::Scissors,
                HandGesture::Rock => HandGesture::Paper,
                HandGesture::Scissors => HandGesture::Rock,
            },
            GameOutcome::Draw => *opponent,
        }
    }
}

impl GameOutcome {
    pub fn parse_outcome_part2(c: u8) -> Self {
        match c {
            b'X' => Self::Lose,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => panic!("Invalid code for outcome: '{}'.", c),
        }
    }
}
