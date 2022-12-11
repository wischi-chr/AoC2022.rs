use std::vec;

use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{NormalizeLineBreaks, RangeIteratorInclusive},
};

#[derive(Default)]
pub struct Day8;

impl PuzzleSolver for Day8 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, _part: PuzzlePart) -> String {
        let mut forest = build_forest(input);

        match _part {
            PuzzlePart::Part1 => {
                forest.mark_all_visible_trees();
                forest.count_visible_trees().to_string()
            }
            PuzzlePart::Part2 => forest.get_max_scenic_score().to_string(),
        }
    }
}

fn build_forest(input: &mut dyn Iterator<Item = u8>) -> Forest2D {
    let chars = input.normalize_line_breaks();

    let mut trees = vec![];

    let mut current_width = 0;
    let mut reference_width = None;
    let mut height = 0;

    for c in chars {
        if c == b'\n' {
            match reference_width {
                None => reference_width = Some(current_width),
                Some(w) => assert!(w == current_width),
            };

            current_width = 0;
            height += 1;
            continue;
        }

        current_width += 1;
        let c = c - b'0'; // convert ASCII to integer
        assert!(c <= 9);

        trees.push(Tree {
            height: c,
            visible: false,
        });
    }

    Forest2D {
        trees,
        height,
        width: reference_width.unwrap(),
    }
}

struct Tree {
    height: u8,
    visible: bool,
}

struct Forest2D {
    trees: Vec<Tree>,
    height: usize,
    width: usize,
}

impl Forest2D {
    pub fn mark_all_visible_trees(&mut self) {
        // iterate rows and check left to right and right to left
        for y in 0..self.height {
            let left_to_right = self.direction_index_iter(0, y, Direction::Right);
            let right_to_left = self.direction_index_iter(self.width - 1, y, Direction::Left);

            self.mark_visible(left_to_right);
            self.mark_visible(right_to_left);
        }

        // iterate columns and check top to bottom and bottom to top
        for x in 0..self.width {
            let top_to_bottom = self.direction_index_iter(x, 0, Direction::Down);
            let bottom_to_top = self.direction_index_iter(x, self.height - 1, Direction::Up);

            self.mark_visible(top_to_bottom);
            self.mark_visible(bottom_to_top);
        }
    }

    fn mark_visible<R: Iterator<Item = usize>>(&mut self, index_range: R) {
        let mut max_tree_height = None;

        for i in index_range {
            let tree = &mut self.trees[i];

            match max_tree_height {
                None => {
                    max_tree_height = Some(tree.height);
                    tree.visible = true;
                }
                Some(h) => {
                    if tree.height > h {
                        tree.visible = true;
                        max_tree_height = Some(tree.height);
                    }
                }
            }
        }
    }

    pub fn count_visible_trees(&self) -> u32 {
        self.trees
            .iter()
            .map(|t| if t.visible { 1 } else { 0 })
            .sum()
    }

    pub fn get_max_scenic_score(&self) -> u32 {
        let mut max_score = 0;

        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                let score = self.get_scenic_score_for_position(x, y);
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }

    fn get_scenic_score_for_position(&self, x: usize, y: usize) -> u32 {
        let vd_right = self.count_trees_in_line(self.direction_index_iter(x, y, Direction::Right));
        let vd_left = self.count_trees_in_line(self.direction_index_iter(x, y, Direction::Left));
        let vd_up = self.count_trees_in_line(self.direction_index_iter(x, y, Direction::Up));
        let vd_down = self.count_trees_in_line(self.direction_index_iter(x, y, Direction::Down));

        return vd_right * vd_left * vd_down * vd_up;
    }

    fn count_trees_in_line<R: Iterator<Item = usize>>(&self, index_range: R) -> u32 {
        let mut tree_height_iter = index_range.map(|i| self.trees[i].height);
        let my_tree_height = tree_height_iter.next().unwrap();

        let mut visible_trees = 0;

        for height in tree_height_iter {
            visible_trees += 1;

            if height >= my_tree_height {
                break;
            }
        }

        visible_trees
    }

    fn direction_index_iter(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> RangeIteratorInclusive<usize> {
        let start = y * self.width + x;

        match direction {
            Direction::Right => RangeIteratorInclusive::new(start, self.get_right(y), 1),
            Direction::Left => RangeIteratorInclusive::new(start, self.get_left(y), 1),
            Direction::Down => RangeIteratorInclusive::new(start, self.get_bottom(x), self.width),
            Direction::Up => RangeIteratorInclusive::new(start, self.get_top(x), self.width),
        }
        .unwrap()
    }

    fn get_left(&self, row: usize) -> usize {
        self.width * row
    }

    fn get_right(&self, row: usize) -> usize {
        self.width * (row + 1) - 1
    }

    fn get_top(&self, column: usize) -> usize {
        column
    }

    fn get_bottom(&self, column: usize) -> usize {
        column + (self.height - 1) * self.width
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}
