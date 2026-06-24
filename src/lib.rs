//! # index-to-position — convert a string index to a line and column
//!
//! Given a byte offset into a string, find its line and column. Useful for turning a parser
//! or lexer error offset into a human-readable `line:column`. A Rust port of the
//! [`index-to-position`](https://www.npmjs.com/package/index-to-position) npm package.
//!
//! ```
//! use index_to_position::index_to_position;
//!
//! let text = "foo\nbar\nbaz";
//! let pos = index_to_position(text, 5); // the 'a' in "bar"
//! assert_eq!((pos.line, pos.column), (1, 1));
//! ```
//!
//! Indices are **byte offsets** (the idiomatic Rust string index, as produced by `str`
//! slicing and most parsers), and lines and columns are zero-based by default. Use
//! [`index_to_position_with`] for one-based output, and [`PositionFinder`] when you need to
//! resolve many indices into the same text efficiently.
//!
//! ```
//! use index_to_position::{index_to_position_with, Options};
//!
//! let pos = index_to_position_with("foo\nbar", 4, Options::new().one_based(true));
//! assert_eq!((pos.line, pos.column), (2, 1));
//! ```
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/index-to-position/0.1.0")]

extern crate alloc;

use alloc::vec::Vec;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// A line and column within a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// The line number (zero-based by default).
    pub line: usize,
    /// The column, as a byte offset within the line (zero-based by default).
    pub column: usize,
}

/// Whether the line and/or column should be one-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Options {
    one_based_line: bool,
    one_based_column: bool,
}

impl Options {
    /// Default options: zero-based line and column.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Make both the line and column one-based.
    #[must_use]
    pub fn one_based(mut self, value: bool) -> Self {
        self.one_based_line = value;
        self.one_based_column = value;
        self
    }

    /// Make the line one-based.
    #[must_use]
    pub fn one_based_line(mut self, value: bool) -> Self {
        self.one_based_line = value;
        self
    }

    /// Make the column one-based.
    #[must_use]
    pub fn one_based_column(mut self, value: bool) -> Self {
        self.one_based_column = value;
        self
    }

    fn line_offset(self) -> usize {
        usize::from(self.one_based_line)
    }

    fn column_offset(self) -> usize {
        usize::from(self.one_based_column)
    }
}

/// Convert a byte `index` into `text` to a zero-based [`Position`].
///
/// # Panics
/// Panics if `index` is greater than `text.len()`.
///
/// ```
/// # use index_to_position::index_to_position;
/// assert_eq!(index_to_position("a\nb", 2).line, 1);
/// ```
#[must_use]
pub fn index_to_position(text: &str, index: usize) -> Position {
    index_to_position_with(text, index, Options::new())
}

/// Convert a byte `index` into `text` to a [`Position`] with the given [`Options`].
///
/// # Panics
/// Panics if `index` is greater than `text.len()`.
#[must_use]
#[allow(clippy::naive_bytecount)] // Keep the crate dependency-free; this is O(n) once per call.
pub fn index_to_position_with(text: &str, index: usize, options: Options) -> Position {
    assert!(
        index <= text.len(),
        "index out of bounds: the length is {} but the index is {index}",
        text.len()
    );

    let bytes = text.as_bytes();

    // The last newline strictly before `index`.
    let line_break_before = if index == 0 {
        None
    } else {
        bytes[..index].iter().rposition(|&b| b == b'\n')
    };

    match line_break_before {
        None => Position {
            line: options.line_offset(),
            column: index + options.column_offset(),
        },
        Some(newline) => {
            let line =
                bytes[..=newline].iter().filter(|&&b| b == b'\n').count() + options.line_offset();
            Position {
                line,
                column: index - newline - 1 + options.column_offset(),
            }
        }
    }
}

/// Resolves many byte indices into the same text efficiently.
///
/// Building a `PositionFinder` precomputes the offset of every line start, so each
/// [`position`](PositionFinder::position) lookup is `O(log lines)` instead of `O(text len)`.
///
/// ```
/// use index_to_position::PositionFinder;
///
/// let finder = PositionFinder::new("foo\nbar\nbaz");
/// assert_eq!((finder.position(0).line, finder.position(9).line), (0, 2));
/// ```
#[derive(Debug, Clone)]
pub struct PositionFinder<'a> {
    text: &'a str,
    /// Byte offset of the start of each line (always begins with `0`).
    line_starts: Vec<usize>,
    options: Options,
}

impl<'a> PositionFinder<'a> {
    /// Build a finder over `text` producing zero-based positions.
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Self::with_options(text, Options::new())
    }

    /// Build a finder over `text` with the given [`Options`].
    #[must_use]
    pub fn with_options(text: &'a str, options: Options) -> Self {
        let mut line_starts = Vec::new();
        line_starts.push(0);
        for (offset, &byte) in text.as_bytes().iter().enumerate() {
            if byte == b'\n' {
                line_starts.push(offset + 1);
            }
        }
        Self {
            text,
            line_starts,
            options,
        }
    }

    /// Resolve a byte `index` to a [`Position`].
    ///
    /// # Panics
    /// Panics if `index` is greater than the text length.
    #[must_use]
    pub fn position(&self, index: usize) -> Position {
        assert!(
            index <= self.text.len(),
            "index out of bounds: the length is {} but the index is {index}",
            self.text.len()
        );
        // The last line start at or before `index`.
        let line = self.line_starts.partition_point(|&start| start <= index) - 1;
        Position {
            line: line + self.options.line_offset(),
            column: index - self.line_starts[line] + self.options.column_offset(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_based() {
        let s = "foo\nbar\r\nbaz\n";
        assert_eq!(index_to_position(s, 0), Position { line: 0, column: 0 });
        assert_eq!(index_to_position(s, 3), Position { line: 0, column: 3 });
        assert_eq!(index_to_position(s, 4), Position { line: 1, column: 0 });
        assert_eq!(index_to_position(s, 8), Position { line: 1, column: 4 });
        assert_eq!(index_to_position(s, 9), Position { line: 2, column: 0 });
        assert_eq!(index_to_position(s, 13), Position { line: 3, column: 0 });
    }

    #[test]
    fn one_based() {
        let o = Options::new().one_based(true);
        assert_eq!(
            index_to_position_with("foo\nbar", 0, o),
            Position { line: 1, column: 1 }
        );
        assert_eq!(
            index_to_position_with("foo\nbar", 4, o),
            Position { line: 2, column: 1 }
        );
    }

    #[test]
    fn separate_offsets() {
        let s = "ab\ncde\nf";
        assert_eq!(
            index_to_position_with(s, 5, Options::new().one_based_line(true)),
            Position { line: 2, column: 2 }
        );
        assert_eq!(
            index_to_position_with(s, 5, Options::new().one_based_column(true)),
            Position { line: 1, column: 3 }
        );
    }

    #[test]
    fn at_end_and_empty() {
        assert_eq!(
            index_to_position("ab\ncde\nf", 8),
            Position { line: 2, column: 1 }
        );
        assert_eq!(index_to_position("", 0), Position { line: 0, column: 0 });
    }

    #[test]
    fn multibyte_byte_offsets() {
        // "é" is two bytes; the 'b' after it begins at byte 2.
        let s = "é\nb"; // bytes: C3 A9 0A 62
        assert_eq!(index_to_position(s, 3), Position { line: 1, column: 0 });
        assert_eq!(index_to_position(s, 0), Position { line: 0, column: 0 });
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_panics() {
        let _ = index_to_position("abc", 4);
    }

    #[test]
    fn finder_matches_function() {
        let s = "foo\nbar\r\nbaz\n\nlast";
        let finder = PositionFinder::new(s);
        for i in 0..=s.len() {
            assert_eq!(finder.position(i), index_to_position(s, i), "index {i}");
        }
    }

    #[test]
    fn finder_one_based() {
        let finder = PositionFinder::with_options("a\nb", Options::new().one_based(true));
        assert_eq!(finder.position(2), Position { line: 2, column: 1 });
    }
}
