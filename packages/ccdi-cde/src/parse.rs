//! Parsing common data elements and their members from documentation.

use std::iter::Peekable;
use std::str::Lines;

/// Conventional line endings in text files (platform-specific).
#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";

/// Conventional line endings in text files (platform-specific).
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";

pub mod cde;

/// Reads from an iterator over lines and trims the concatenates lines together.
/// This is useful when you want to treat a multiline comment as a single
/// [`String`].
///
/// Note that, if the contiguous lines are postceded by an empty line, then the
/// postceding empty line is also consumed. This is convenient for calling this
/// function multiple times in a row on the same iterator.
///
/// # Examples
///
/// ```
/// use ccdi_cde as cde;
///
/// use cde::parse::trim_and_concat_contiguous_lines;
///
/// let mut lines = "hello\nthere,\nworld\n\nfoo\nbar\n\n\"baz\ntest\"".lines().peekable();
///
/// assert_eq!(
///     trim_and_concat_contiguous_lines(&mut lines),
///     Some(String::from("hello there, world"))
/// );
///
/// assert_eq!(
///     trim_and_concat_contiguous_lines(&mut lines),
///     Some(String::from("foo bar"))
/// );
///
/// assert_eq!(
///     trim_and_concat_contiguous_lines(&mut lines),
///     Some(String::from(r#""baz test""#))
/// );
///
/// assert_eq!(
///     trim_and_concat_contiguous_lines(&mut lines),
///     None
/// );
/// ```
pub fn trim_and_concat_contiguous_lines(lines: &mut Peekable<Lines<'_>>) -> Option<String> {
    // If the first line is `None`, return `None`;
    lines.peek()?;

    let mut results = vec![];

    for line in lines.by_ref() {
        let line = line.trim();

        // Consume the postceding empty line.
        if line.is_empty() {
            break;
        }

        results.push(line);
    }

    Some(results.join(" ").trim().to_string())
}
