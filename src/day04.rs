use crate::common::{LfEofDropable, LineSplittable, NormalizeLineBreaks};

pub fn solve<I>(input: &mut I) -> (String, String)
where
    I: Iterator<Item = u8>,
{
    let lines = input
        .normalize_line_breaks()
        .split_lf_line_breaks()
        .drop_lf_eof();

    let mut contains_count = 0;
    let mut overlap_count = 0;

    for line in lines {
        let (a, b) = parse_double_range(&line);

        if Range::one_contains_other(&a, &b) {
            contains_count += 1;
        }

        if a.overlaps_with(&b) {
            overlap_count += 1;
        }
    }

    (contains_count.to_string(), overlap_count.to_string())
}

fn parse_double_range(data: &[u8]) -> (Range, Range) {
    let (a, b) = split(b',', data);
    (parse_range(a), parse_range(b))
}

fn parse_range(data: &[u8]) -> Range {
    let (a, b) = split(b'-', data);

    Range {
        start: parse_integer(a),
        end: parse_integer(b),
    }
}

fn parse_integer(data: &[u8]) -> i32 {
    std::str::from_utf8(data)
        .expect("data should be valid ASCII")
        .parse::<i32>()
        .expect("data should be a valid integer")
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
