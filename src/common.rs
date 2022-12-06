use std::iter::Peekable;

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
                    if let Some(b'\n') = self.iter.peek() {
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
        Self: Sized,
        Self: Iterator<Item = u8>;
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
        Self: Sized,
        Self: Iterator<Item = u8>;
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
        Self: Sized,
        Self: Iterator<Item = Vec<u8>>;
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
            if x.len() == 0 && self.iter.peek() == None {
                // the current element is empty (zero length vector)
                // and the last element so we drop it and directly return None.
                return None;
            }
        }

        item
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

    use super::LineSplittable;

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
