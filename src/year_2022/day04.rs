use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{parse, LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day4;

impl PuzzleSolver for Day4 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let lines = input
            .normalize_line_breaks()
            .split_lf_line_breaks()
            .drop_lf_eof();

        let mut count = 0;

        for line in lines {
            let (a, b) = parse_double_range(&line);

            if match part {
                PuzzlePart::Part1 => Range::one_contains_other(&a, &b),
                PuzzlePart::Part2 => a.overlaps_with(&b),
            } {
                count += 1;
            }
        }

        count.to_string()
    }
}

fn parse_double_range(data: &[u8]) -> (Range, Range) {
    let (a, b) = split(b',', data);
    (parse_range(a), parse_range(b))
}

fn parse_range(data: &[u8]) -> Range {
    let (a, b) = split(b'-', data);

    Range {
        start: parse(a),
        end: parse(b),
    }
}

fn split(delimiter: u8, data: &[u8]) -> (&[u8], &[u8]) {
    let index = data
        .iter()
        .position(|&x| x == delimiter)
        .expect("data should have delimiter at least once");

    (&data[0..index], &data[index + 1..])
}

struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    const fn one_contains_other(a: &Self, b: &Self) -> bool {
        a.contains(b) || b.contains(a)
    }

    const fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    const fn overlaps_with(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (self.start >= other.start && self.start <= other.end)
    }
}
