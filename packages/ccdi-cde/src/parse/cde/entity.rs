//! Parsing an entity for common data elements.

use std::iter::Peekable;
use std::str::Lines;

use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::parse::trim_and_concat_contiguous_lines;

const STANDARD_PATTERN: &str = r"^\*\*`(?P<standard>.*?)`\*\*$";
const URL_PATTERN: &str = r"^Link: <(?P<url>.*)>$";

/// A error related to parsing an [`Entity`].
#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Attempted to parse a field with no documentation.
    Empty,

    /// The iterator ended early. The argument is a plain-text description of
    /// the part of the docstring that was being parsed when the iterator
    /// stopped.
    IteratorEndedEarly(String),

    /// The standard line was does not match the format we expect. The
    /// argument is the line that we are attempting to parse.
    InvalidStandardFormat(String),

    /// A field URL line was does not match the format we expect. The
    /// argument is the line that we are attempting to parse.
    InvalidURLFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "cannot parse entity with no documentation"),
            ParseError::IteratorEndedEarly(entity) => {
                write!(f, "iterator ended early when parsing {entity}")
            }
            ParseError::InvalidStandardFormat(value) => {
                write!(
                    f,
                    "entity's standard line does not match expected format: \"{value}\". \
                     The following format is expected: \"**`STANDARD`**\""
                )
            }
            ParseError::InvalidURLFormat(value) => {
                write!(
                    f,
                    "entity's URL line does not match expected format: \"{value}\". \
                    The following format is expected: \"Link: <URL>\""
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// A [`Result`](std::result::Result) with a [`ParseError`].
pub type Result<T> = std::result::Result<T, ParseError>;

/// A parsed entity that describes a common data element. An entity is either a
/// `struct` or an `enum` (both can be used to describe common data elements).
#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    standard: String,
    description: String,
    url: String,
}

impl Entity {
    /// Gets the standard for the [`Entity`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::Entity;
    ///
    /// let entity = r#"**`A Standard`**
    ///
    /// A description that spans
    /// multiple lines.
    ///
    /// Link: <https://example.com>"#
    ///     .parse::<Entity>()?;
    ///
    /// assert_eq!(entity.standard(), "A Standard");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn standard(&self) -> &str {
        self.standard.as_str()
    }

    /// Gets the description for the [`Entity`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::Entity;
    ///
    /// let entity = r#"**`A Standard`**
    ///
    /// A description that spans
    /// multiple lines.
    ///
    /// Link: <https://example.com>"#
    ///     .parse::<Entity>()?;
    ///
    /// assert_eq!(
    ///     entity.description(),
    ///     "A description that spans multiple lines."
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Gets the URL for the [`Entity`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::Entity;
    ///
    /// let entity = r#"**`A Standard`**
    ///
    /// A description that spans
    /// multiple lines.
    ///
    /// Link: <https://example.com>"#
    ///     .parse::<Entity>()?;
    ///
    /// assert_eq!(entity.url(), "https://example.com");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}

impl std::str::FromStr for Entity {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().peekable();

        if lines.peek().is_none() {
            return Err(ParseError::Empty);
        }

        let standard = parse_standard(&mut lines)?;
        let description = parse_description(&mut lines)?;
        let url = parse_url(&mut lines)?;

        Ok(Self {
            standard,
            description,
            url,
        })
    }
}

fn parse_standard(lines: &mut Peekable<Lines<'_>>) -> Result<String> {
    let line = trim_and_concat_contiguous_lines(lines)
        .map(Ok)
        .unwrap_or(Err(ParseError::IteratorEndedEarly(String::from(
            "standard",
        ))))?;

    // SAFETY: we test that this pattern unwraps statically below.
    let regex = Regex::new(STANDARD_PATTERN).unwrap();

    match regex.captures(line.as_str()) {
        Some(captures) => {
            // SAFETY: this key is tested for existence in the regex below, so
            // it will always be present.
            Ok(captures.name("standard").unwrap().as_str().to_string())
        }
        None => Err(ParseError::InvalidStandardFormat(line.to_owned())),
    }
}

fn parse_description(lines: &mut Peekable<Lines<'_>>) -> Result<String> {
    trim_and_concat_contiguous_lines(lines)
        .map(Ok)
        .unwrap_or(Err(ParseError::IteratorEndedEarly(String::from(
            "description",
        ))))
}

fn parse_url(lines: &mut Peekable<Lines<'_>>) -> Result<String> {
    let line = match trim_and_concat_contiguous_lines(lines) {
        Some(line) => line,
        None => return Err(ParseError::IteratorEndedEarly(String::from("url"))),
    };

    // SAFETY: we test that this pattern unwraps statically below.
    let regex = Regex::new(URL_PATTERN).unwrap();

    match regex.captures(line.as_str()) {
        // SAFETY: we test that the 'url' named group exists in the tests below,
        // so this will always be present.
        Some(captures) => Ok(captures.name("url").unwrap().as_str().to_string()),
        None => Err(ParseError::InvalidURLFormat(line.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn the_standard_pattern_compiles_and_captures() {
        let regex = Regex::new(STANDARD_PATTERN).unwrap();

        let captures = regex.captures("**`caDSR CDE 12217251 v1.00`**").unwrap();
        assert_eq!(
            captures.name("standard").unwrap().as_str(),
            "caDSR CDE 12217251 v1.00"
        );
    }

    #[test]
    fn the_url_pattern_compiles_and_captures() {
        let regex = Regex::new(URL_PATTERN).unwrap();
        let captures = regex.captures("Link: <https://test.com>").unwrap();

        assert_eq!(captures.name("url").unwrap().as_str(), "https://test.com");
    }

    #[test]
    fn it_parses_a_multiline_standard() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let entity = r#"**`A Standard
        That Spans Multiple Lines`**
    
        A description that spans
        multiple lines.
    
        Link: <https://example.com>"#
            .parse::<Entity>()?;

        assert_eq!(entity.standard(), "A Standard That Spans Multiple Lines");

        Ok(())
    }

    #[test]
    fn it_parses_a_multiline_url() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let entity = r#"**`A Standard`**
    
        A description that spans
        multiple lines.
    
        Link:
        <https://example.com>"#
            .parse::<Entity>()?;

        assert_eq!(entity.url(), "https://example.com");

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_field_with_no_documentation(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "".parse::<Entity>().unwrap_err();
        assert_eq!(err, ParseError::Empty);

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_field_with_a_missing_description(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = r#"**`A Standard`**
        "#
        .parse::<Entity>()
        .unwrap_err();

        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("description"))
        );

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_field_with_a_missing_url(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = r#"**`A Standard`**

        A description.
        "#
        .parse::<Entity>()
        .unwrap_err();

        assert_eq!(err, ParseError::IteratorEndedEarly(String::from("url")));

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_an_incorrectly_formatted_standard(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = r#"A Standard
    
        A description that spans
        multiple lines.
    
        Link: <https://example.com>"#
            .parse::<Entity>()
            .unwrap_err();

        assert_eq!(
            err,
            ParseError::InvalidStandardFormat(String::from("A Standard"))
        );

        assert_eq!(err.to_string(), "entity's standard line does not match expected format: \"A Standard\". The following format is expected: \"**`STANDARD`**\"");

        // Ensure that we must have code backticks in the standard name.

        let err = r#"**A Standard**
    
        A description that spans
        multiple lines.
    
        Link: <https://example.com>"#
            .parse::<Entity>()
            .unwrap_err();

        assert_eq!(
            err,
            ParseError::InvalidStandardFormat(String::from("**A Standard**"))
        );

        assert_eq!(err.to_string(), "entity's standard line does not match expected format: \"**A Standard**\". The following format is expected: \"**`STANDARD`**\"");

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_an_incorrectly_formatted_url(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = r#"**`A Standard`**
    
        A description that spans
        multiple lines.
    
        Link:<https://example.com>"#
            .parse::<Entity>()
            .unwrap_err();

        assert_eq!(
            err,
            ParseError::InvalidURLFormat(String::from("Link:<https://example.com>"))
        );

        assert_eq!(err.to_string(), "entity's URL line does not match expected format: \"Link:<https://example.com>\". The following format is expected: \"Link: <URL>\"");

        // Ensure that we must have code backticks in the standard name.

        let err = r#"**`A Standard`**
    
        A description that spans
        multiple lines.
    
        Link: https://example.com"#
            .parse::<Entity>()
            .unwrap_err();

        assert_eq!(
            err,
            ParseError::InvalidURLFormat(String::from("Link: https://example.com"))
        );

        assert_eq!(err.to_string(), "entity's URL line does not match expected format: \"Link: https://example.com\". The following format is expected: \"Link: <URL>\"");

        Ok(())
    }
}
