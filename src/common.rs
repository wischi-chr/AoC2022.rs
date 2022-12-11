use std::{iter::Peekable, str::FromStr};

use num::{
    traits::ops::overflowing::{OverflowingAdd, OverflowingSub},
    Integer,
};

pub struct LineBreakNormalizer<I>
where
    I: Iterator<Item = u8>,
{
    iter: Peekable<I>,
}

impl<I> Iterator for LineBreakNormalizer<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // This iterator converts all CR (legacy Mac) and CRLF (Windows) into LF line breaks.
        // - if the read byte is `\r` and the next byte is a `\n` than just drop it
        // - if the read byte is `\r` and the next byte is not `\n` than emit `\n` instead.
        // - in all other cases emit the original byte

        match self.iter.next() {
            Some(byte) => {
                if byte == b'\r' {
                    if self.iter.peek() == Some(&b'\n') {
                        self.iter.next();
                    }
                    Some(b'\n')
                } else {
                    Some(byte)
                }
            }
            None => None,
        }
    }
}

impl<I> LineBreakNormalizer<I>
where
    I: Iterator<Item = u8>,
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

pub trait NormalizeLineBreaks {
    /// Implements the ability to drop `\r\n` byte pairs from a stream, converting each instance to a single `\n`.
    fn normalize_line_breaks(self) -> LineBreakNormalizer<Self>
    where
        Self: Sized + Iterator<Item = u8>;
}

impl<I> NormalizeLineBreaks for I
where
    I: Iterator<Item = u8>,
{
    fn normalize_line_breaks(self) -> LineBreakNormalizer<Self> {
        LineBreakNormalizer::new(self)
    }
}

pub trait LineSplittable {
    fn split_lf_line_breaks(self) -> LineSplitter<Self>
    where
        Self: Sized + Iterator<Item = u8>;
}

impl<I> LineSplittable for I
where
    I: Iterator<Item = u8>,
{
    fn split_lf_line_breaks(self) -> LineSplitter<Self> {
        LineSplitter {
            iter: self,
            ended: false,
        }
    }
}

impl<I> LfEofDropable for I
where
    I: Iterator<Item = Vec<u8>>,
{
    fn drop_lf_eof(self) -> LfEofDropper<Self> {
        LfEofDropper {
            iter: self.peekable(),
        }
    }
}

pub trait LfEofDropable {
    fn drop_lf_eof(self) -> LfEofDropper<Self>
    where
        Self: Sized + Iterator<Item = Vec<u8>>;
}

pub struct LfEofDropper<I>
where
    I: Iterator<Item = Vec<u8>>,
{
    iter: Peekable<I>,
}

impl<I> Iterator for LfEofDropper<I>
where
    I: Iterator<Item = Vec<u8>>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();

        if let Some(x) = &item {
            if x.is_empty() && self.iter.peek().is_none() {
                // the current element is empty (zero length vector)
                // and the last element so we drop it and directly return None.
                return None;
            }
        }

        item
    }
}

pub struct RangeIteratorInclusive<T> {
    start: T,
    stop: T,
    step: T,
    current: Option<T>,
}

impl<T> Iterator for RangeIteratorInclusive<T>
where
    T: OverflowingAdd + Copy + PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                self.current = Some(self.start);
                self.current
            }
            Some(c) => {
                if c == self.stop {
                    None
                } else {
                    // Note: this iterator does negative steps by overflow adding "very large" positive values
                    let (next, _) = c.overflowing_add(&self.step);
                    self.current = Some(next);
                    self.current
                }
            }
        }
    }
}

impl<T> RangeIteratorInclusive<T>
where
    T: Copy,
{
    /// Note that step MUST NOT be negative or zero (will return None).
    pub fn new(start: T, stop: T, step: T) -> Option<Self>
    where
        T: OverflowingSub + Integer,
    {
        if step <= T::zero() {
            return None;
        }

        let decrement = start > stop;

        let distance = if decrement {
            start - stop
        } else {
            stop - start
        };

        let step_count = distance.div_floor(&step);
        let normalized_distance = step_count * step;

        // the "normalized_stop" describes the last value the iterator lands exactly.
        // for example it the consumer calls this function like so: new(1, 11, 4)
        // the output will be 1, 5, 9 and the last value (in this case 9) is the normalized_stop value
        // we do that so simplify the implementation of next()

        let normalized_stop = if decrement {
            start - normalized_distance
        } else {
            start + normalized_distance
        };

        let mut step = step;

        if decrement {
            (step, _) = T::zero().overflowing_sub(&step);
        }

        Some(Self {
            current: None,
            start,
            stop: normalized_stop,
            step,
        })
    }
}

pub fn parse<T: FromStr>(data: &[u8]) -> T {
    match std::str::from_utf8(data).unwrap().parse::<T>() {
        Err(_) => panic!("Failed to parse data"),
        Ok(x) => x,
    }
}

pub struct LineSplitter<I>
where
    I: Iterator<Item = u8>,
{
    iter: I,
    ended: bool,
}

impl<I> Iterator for LineSplitter<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        let mut result = vec![];

        loop {
            match self.iter.next() {
                None => {
                    self.ended = true;
                    return Some(result);
                }
                Some(b'\n') => {
                    return Some(result);
                }
                Some(x) => result.push(x),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{LfEofDropable, NormalizeLineBreaks};
    use std::io::{Cursor, Read};

    use super::{LineSplittable, RangeIteratorInclusive};

    #[test]
    fn range_iterator_works_for_trivial_cases() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let observed = RangeIteratorInclusive::new(1, 9, 1)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_works_if_step_doesnt_hit_the_stop_value_exactly() {
        let expected = vec![1, 5, 9];
        let observed = RangeIteratorInclusive::new(1, 12, 4)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_works_for_large_positive_steps_in_u32() {
        let expected: Vec<u32> = vec![0, u32::MAX];
        let observed = RangeIteratorInclusive::new(0u32, u32::MAX, u32::MAX)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_works_for_large_negative_steps_in_u32() {
        let expected: Vec<u32> = vec![u32::MAX, 0];
        let observed = RangeIteratorInclusive::new(u32::MAX, 0u32, u32::MAX)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_works_when_passing_though_zero() {
        let expected = vec![-7, -4, -1, 2, 5];
        let observed = RangeIteratorInclusive::new(-7, 7, 3)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_can_walk_backwards() {
        let expected = vec![5, 4, 3, 2, 1];
        let observed = RangeIteratorInclusive::new(5, 1, 1)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn range_iterator_can_walk_backwards_u8() {
        let expected: Vec<u8> = vec![5, 4, 3, 2, 1];
        let observed = RangeIteratorInclusive::new(5u8, 1u8, 1u8)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(expected, observed);
    }

    #[test]
    fn normalizing_line_breaks_work() {
        let input = b"some\rtest\r\r\nwith\ndifferent\n\nline\r\nbreak\nstyles";
        let expected = b"some\ntest\n\nwith\ndifferent\n\nline\nbreak\nstyles".to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .normalize_line_breaks()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn line_break_before_eof_is_converted() {
        let input = b"test\r";
        let expected = b"test\n".to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .normalize_line_breaks()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn splitting_lines_works() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn lf_eof_drop_doesnt_drop_empty_lines_that_are_not_at_the_end() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .drop_lf_eof()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn lf_eof_drop_drops_an_empty_line_before_eof() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks\n";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .drop_lf_eof()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn lf_eof_drop_doesnt_do_anything_if_the_last_element_is_not_an_empty_line() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .drop_lf_eof()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn lf_eof_drop_doesnt_drop_the_last_element_if_it_contains_whitespace() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks\n ";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
            b" ".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .drop_lf_eof()
            .collect();

        assert_eq!(expected, processed);
    }

    #[test]
    fn lf_eof_drop_only_drops_a_single_empty_line_at_the_end() {
        let input = b"some\ntest\n\nwith\nmultiple line\nbreaks\n\n";

        let expected = [
            b"some".to_vec(),
            b"test".to_vec(),
            b"".to_vec(),
            b"with".to_vec(),
            b"multiple line".to_vec(),
            b"breaks".to_vec(),
            b"".to_vec(),
        ]
        .to_vec();

        let processed: Vec<_> = Cursor::new(input)
            .bytes()
            .map(|x| x.unwrap())
            .split_lf_line_breaks()
            .drop_lf_eof()
            .collect();

        assert_eq!(expected, processed);
    }
}
