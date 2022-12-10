use crate::{
    aoc_general::{PuzzlePart, PuzzleSolver},
    common::{parse, LfEofDropable, LineSplittable, NormalizeLineBreaks},
};

#[derive(Default)]
pub struct Day5;

impl PuzzleSolver for Day5 {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let mut lines = input
            .normalize_line_breaks()
            .split_lf_line_breaks()
            .drop_lf_eof()
            .peekable();

        // We use the fact that the input at the top is always padded with spaces to be a constant width.
        // each column has a width of three chars and a space in between. So we use some simple math to
        // determine the number of columns based on the width of the first line with (width - 1) / 4

        let expanded_width = lines
            .peek()
            .expect("input should at least have a single line")
            .len()
            + 1;

        assert!(expanded_width % 4 == 0);
        let column_count = expanded_width / 4;

        let mut columns = Vec::<Vec<u8>>::with_capacity(column_count);

        // initialize empty vecs for each column
        for _ in 0..column_count {
            columns.push(vec![]);
        }

        // initialize stacks
        loop {
            let line = lines.next().expect("line should contain stack data");

            if line[1] == b'1' && line[0] == b' ' {
                // found line with column numbers, so break
                break;
            }

            // add crates (if any) for this row to respective columns
            for (i, column) in columns.iter_mut().enumerate() {
                let char_pos = 4 * i + 1;

                if line[char_pos - 1] == b'[' && line[char_pos + 1] == b']' {
                    // container
                    column.push(line[char_pos]);
                }
            }
        }

        // reverse vectors so the top ones (we added first) are at the end of the list
        columns.iter_mut().for_each(|c| c.reverse());

        // skip and assert empty line
        let line = lines.next().unwrap();
        assert!(line.is_empty());

        let mut temp_column = vec![];

        // apply moves to current state
        loop {
            let line = lines.next();

            if line.is_none() {
                // found end of file/input
                break;
            }

            // destruct move instruction
            let line = line.unwrap();
            let mut parts = line.split(|x| *x == b' ');
            assert!(parts.next().unwrap() == b"move");
            let count = parse::<i32>(parts.next().unwrap());
            assert!(parts.next().unwrap() == b"from");
            let source = parse::<usize>(parts.next().unwrap()) - 1;
            assert!(parts.next().unwrap() == b"to");
            let target = parse::<usize>(parts.next().unwrap()) - 1;

            match part {
                PuzzlePart::Part1 => {
                    let (source, target) = borrow_2_mut(&mut columns, source, target);
                    move_crates(source, target, count);
                }
                PuzzlePart::Part2 => {
                    move_crates(&mut columns[source], &mut temp_column, count);
                    move_crates(&mut temp_column, &mut columns[target], count);
                }
            }
        }

        let output: Vec<_> = columns.iter().map(|c| *c.last().unwrap()).collect();

        String::from_utf8(output).unwrap()
    }
}

fn move_crates(source: &mut Vec<u8>, target: &mut Vec<u8>, amount: i32) {
    for _ in 0..amount {
        let item = source.pop().unwrap();
        target.push(item);
    }
}

fn borrow_2_mut<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert!(a != b);

    if b < a {
        let result = borrow_2_mut(slice, b, a);
        return (result.1, result.0);
    }

    let (head, tail) = slice.split_at_mut(b);
    (&mut head[a], &mut tail[0])
}
