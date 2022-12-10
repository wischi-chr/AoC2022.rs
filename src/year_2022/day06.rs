use crate::aoc_general::{PuzzlePart, PuzzleSolver};

#[derive(Default)]
pub struct Day6;

impl PuzzleSolver for Day6 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let window_size = match part {
            PuzzlePart::Part1 => 4,
            PuzzlePart::Part2 => 14,
        };

        let mut window = (0..window_size)
            .into_iter()
            .map(|_| {
                input
                    .next()
                    .expect("input should have at least the same length as the window size.")
            })
            .collect::<Vec<_>>();

        let mut index = window_size;

        loop {
            if all_chars_different(&window) {
                return index.to_string();
            }

            window[index % window_size] = input.next().unwrap();
            index += 1;
        }
    }
}

fn all_chars_different(slice: &[u8]) -> bool {
    let outer_length = slice.len();
    let inner_length = outer_length - 1;

    for i in 0..inner_length {
        for j in i + 1..outer_length {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }

    true
}
