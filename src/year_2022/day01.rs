use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day1;

impl PuzzleSolver for Day1 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let lines = input.normalize_line_breaks().split_lf_line_breaks();
        let mut calories = vec![];
        let mut tmp_calories_sum = 0;

        for line in lines {
            let line = std::str::from_utf8(&line).expect("line should be valid UTF-8");

            if line.is_empty() {
                calories.push(tmp_calories_sum);
                tmp_calories_sum = 0;
                continue;
            }

            let food_calories = line
                .parse::<u32>()
                .expect("Food calories not a valid number.");

            tmp_calories_sum += food_calories;
        }

        calories.sort_by(|a, b| b.cmp(a));

        match part {
            PuzzlePart::Part1 => calories[0].to_string(),
            PuzzlePart::Part2 => calories.iter().take(3).sum::<u32>().to_string(),
        }
    }
}
