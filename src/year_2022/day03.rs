use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day3;

impl PuzzleSolver for Day3 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let mut lines = input
            .normalize_line_breaks()
            .split_lf_line_breaks()
            .drop_lf_eof();

        match part {
            PuzzlePart::Part1 => day03_part1(&mut lines),
            PuzzlePart::Part2 => day03_part2(&mut lines),
        }
    }
}

fn get_priority_for_item(item: u8) -> u8 {
    match item {
        (b'a'..=b'z') => item - b'a' + 1,
        (b'A'..=b'Z') => item - b'A' + 27,
        _ => panic!("Unexpected item '{}'", item as char),
    }
}

fn day03_part1<I>(lines: &mut I) -> String
where
    I: Iterator<Item = Vec<u8>>,
{
    lines
        .map(|x| {
            let len = x.len();

            assert!(len % 2 == 0);
            assert!(len >= 2);

            let half = len / 2;

            let compartment1 = &x[0..half];
            let compartment2 = &x[half..];

            let positions =
                find_duplicate(compartment1, compartment2).expect("No duplicate item found");

            let item = &compartment1[positions.0];

            i32::from(get_priority_for_item(*item))
        })
        .sum::<i32>()
        .to_string()
}

fn day03_part2<I>(lines: &mut I) -> String
where
    I: Iterator<Item = Vec<u8>>,
{
    lines
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut map = HashMap::new();

            for group_member in group {
                for b in group_member.iter().unique() {
                    map.entry(*b).and_modify(|x| *x += 1).or_insert(1);
                }
            }

            let item = *map.iter().find(|x| *x.1 == 3).unwrap().0;
            i32::from(get_priority_for_item(item))
        })
        .sum::<i32>()
        .to_string()
}

fn find_duplicate<T>(a: &[T], b: &[T]) -> Option<(usize, usize)>
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
