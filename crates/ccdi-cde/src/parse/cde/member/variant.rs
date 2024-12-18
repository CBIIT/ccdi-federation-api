//! Parsing the members of `enum`s ("variants") as common data elements.

use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::iter::Peekable;
use std::str::FromStr;
use std::str::Lines;

use indexmap::IndexMap;

use crate::parse::trim_and_concat_contiguous_lines;

const PERMISSIBLE_VALUE_PATTERN: &str = r"^`(?P<permissible_value>.*)`$";
const METADATA_PATTERN: &str = r"^\*\s*\*\*(?P<key>.*)\*\*:\s*(?P<value>.*)$";

/// An error related to parsing a [`Variant`].
#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Attempted to parse a variant with no documentation.
    Empty,

    /// The iterator ended early. The argument is a plain-text description of
    /// the part of the docstring that was being parsed when the iterator
    /// stopped.
    IteratorEndedEarly(String),

    /// A permissible value line was does not match the format we expect. The
    /// argument is the line that we are attempting to parse.
    InvalidPermissibleValueFormat(String),

    /// A variant metadata line was does not match the format we expect. The
    /// argument is the line that we are attempting to parse.
    InvalidMemberMetadataFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "cannot parse variant with no documentation"),
            ParseError::IteratorEndedEarly(entity) => {
                write!(f, "iterator ended early when parsing {entity}")
            }
            ParseError::InvalidPermissibleValueFormat(value) => {
                write!(
                    f,
                    "permissible value does not match expected format: \"{value}\".
                     The following format is expected: \"`PERMISSIBLE VALUE`\""
                )
            }
            ParseError::InvalidMemberMetadataFormat(value) => {
                write!(
                    f,
                    "variant metadata does not match expected format: \"{value}\".
                     The following format is expected: \"* **NAME**: DESCRIPTION\""
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

/// A parsed variant of an `enum` that describes a common data element.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Variant {
    permissible_value: String,
    metadata: Option<IndexMap<String, String>>,
    description: String,
}

impl Variant {
    /// Gets the permissible value for the [`Variant`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Variant;
    ///
    /// let variant = r#"`Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 4266671
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   03/09/2023
    ///
    /// Not known, not observed, not recorded, or refused."#
    ///     .parse::<Variant>()?;
    ///
    /// assert_eq!(variant.permissible_value(), "Unknown");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn permissible_value(&self) -> &str {
        self.permissible_value.as_str()
    }

    /// Gets the metadata map for the [`Variant`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Variant;
    ///
    /// let variant = r#"`Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 4266671
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   03/09/2023
    ///
    /// Not known, not observed, not recorded, or refused."#
    ///     .parse::<Variant>()?;
    ///
    /// let metadata = variant.metadata().unwrap();
    ///
    /// assert_eq!(metadata.get("VM Long Name").unwrap(), "Unknown");
    /// assert_eq!(metadata.get("VM Public ID").unwrap(), "4266671");
    /// assert_eq!(metadata.get("Concept Code").unwrap(), "C17998");
    /// assert_eq!(metadata.get("Begin Date").unwrap(), "03/09/2023");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&IndexMap<String, String>> {
        self.metadata.as_ref()
    }

    /// Gets the description for the [`Variant`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Variant;
    ///
    /// let variant = r#"`Unknown`
    ///
    /// * **VM Long Name**: Unknown
    /// * **VM Public ID**: 4266671
    /// * **Concept Code**: C17998
    /// * **Begin Date**:   03/09/2023
    ///
    /// Not known, not observed, not recorded, or refused."#
    ///     .parse::<Variant>()?;
    ///
    /// assert_eq!(
    ///     variant.description(),
    ///     "Not known, not observed, not recorded, or refused."
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl FromStr for Variant {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().peekable();

        if lines.peek().is_none() {
            return Err(ParseError::Empty);
        }

        let permissible_value = parse_permissible_value(&mut lines)?;
        let metadata = parse_metadata(&mut lines)?;
        let description = parse_description(&mut lines)?;

        Ok(Self {
            permissible_value,
            metadata,
            description,
        })
    }
}

fn parse_permissible_value(lines: &mut Peekable<Lines<'_>>) -> Result<String> {
    let permissible_value = trim_and_concat_contiguous_lines(lines)
        .map(Ok)
        .unwrap_or(Err(ParseError::IteratorEndedEarly(String::from(
            "permissible value",
        ))))?;

    // SAFETY: we test that this pattern unwraps statically below.
    let regex = Regex::new(PERMISSIBLE_VALUE_PATTERN).unwrap();

    regex
        .captures(&permissible_value)
        .and_then(|value| value.name("permissible_value"))
        .map(|match_| Ok(match_.as_str().to_string()))
        .unwrap_or(Err(ParseError::InvalidPermissibleValueFormat(
            permissible_value.to_string(),
        )))
}

fn parse_metadata(lines: &mut Peekable<Lines<'_>>) -> Result<Option<IndexMap<String, String>>> {
    match lines.peek() {
        Some(line) => {
            let line = line.trim();

            if !line.starts_with('*') {
                return Ok(None);
            }
        }
        None => return Err(ParseError::IteratorEndedEarly(String::from("metadata"))),
    }

    // SAFETY: we test that this pattern unwraps statically below.
    let regex = Regex::new(METADATA_PATTERN).unwrap();
    let mut results = IndexMap::<String, String>::new();

    while let Some(line) = lines.next().map(|line| line.trim()) {
        if !line.starts_with('*') {
            break;
        }

        match regex.captures(line) {
            Some(captures) => results.insert(
                // SAFETY: these two keys are tested for existence in the regex
                // below, so they will always be present.
                captures.name("key").unwrap().as_str().to_string(),
                captures.name("value").unwrap().as_str().to_string(),
            ),
            None => return Err(ParseError::InvalidMemberMetadataFormat(line.to_owned())),
        };
    }

    Ok(Some(results))
}

fn parse_description(lines: &mut Peekable<Lines<'_>>) -> Result<String> {
    match trim_and_concat_contiguous_lines(lines) {
        Some(line) => Ok(line.to_owned()),
        None => Err(ParseError::IteratorEndedEarly(String::from("description"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_permissible_value_pattern_compiles_and_captures() {
        let regex = Regex::new(PERMISSIBLE_VALUE_PATTERN).unwrap();
        let captures = regex.captures("`A Permissible Value`").unwrap();

        assert_eq!(
            captures.name("permissible_value").unwrap().as_str(),
            "A Permissible Value"
        );
    }

    #[test]
    fn the_metadata_pattern_compiles_and_captures() {
        let regex = Regex::new(METADATA_PATTERN).unwrap();
        let captures = regex.captures("* **Hello**: World").unwrap();

        assert_eq!(captures.name("key").unwrap().as_str(), "Hello");
        assert_eq!(captures.name("value").unwrap().as_str(), "World");
    }

    #[test]
    fn it_parses_a_variant_with_metadata_correctly(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let value = "`Not Reported`

        * **VM Long Name**: Not Reported
        * **VM Public ID**: 5612322
        * **Concept Code**: C43234
        * **Begin Date**:   10/03/2023
        
        Not provided or available."
            .parse::<Variant>()?;

        assert_eq!(value.permissible_value(), "Not Reported");

        let metadata = value.metadata().unwrap();
        assert_eq!(
            metadata.get("VM Long Name").unwrap().as_str(),
            "Not Reported"
        );
        assert_eq!(metadata.get("VM Public ID").unwrap().as_str(), "5612322");
        assert_eq!(metadata.get("Concept Code").unwrap().as_str(), "C43234");
        assert_eq!(metadata.get("Begin Date").unwrap().as_str(), "10/03/2023");

        assert_eq!(value.description(), "Not provided or available.");

        Ok(())
    }

    #[test]
    fn it_parses_a_variant_with_no_metadata_correctly(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let value = "`Not Reported`

        Not provided or available."
            .parse::<Variant>()?;

        assert_eq!(value.permissible_value(), "Not Reported");
        assert_eq!(value.metadata(), None);
        assert_eq!(value.description(), "Not provided or available.");

        Ok(())
    }

    #[test]
    fn it_parses_a_multiline_permissible_value_correctly(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let value = "`Not Reported, alongside
        another
        line.`

        * **VM Long Name**: Not Reported
        * **VM Public ID**: 5612322
        * **Concept Code**: C43234
        * **Begin Date**:   10/03/2023

        Not provided or available."
            .parse::<Variant>()?;

        assert_eq!(
            value.permissible_value(),
            "Not Reported, alongside another line."
        );

        let value = "`Not Reported, alongside
        another
        line.`

        Not provided or available."
            .parse::<Variant>()?;

        assert_eq!(
            value.permissible_value(),
            "Not Reported, alongside another line."
        );

        Ok(())
    }

    #[test]
    fn it_parses_a_multiline_description_correctly(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let value = "`Not Reported`

        * **VM Long Name**: Not Reported
        * **VM Public ID**: 5612322
        * **Concept Code**: C43234
        * **Begin Date**:   10/03/2023

        Not provided or available,
        alongside another
        line."
            .parse::<Variant>()?;

        assert_eq!(
            value.description(),
            "Not provided or available, alongside another line."
        );

        let value = "`Not Reported`

        Not provided or available,
        alongside another
        line."
            .parse::<Variant>()?;

        assert_eq!(
            value.description(),
            "Not provided or available, alongside another line."
        );

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_variant_with_no_documentation(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "".parse::<Variant>().unwrap_err();
        assert_eq!(err, ParseError::Empty);

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_variant_with_missing_metadata(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "`Not Reported`".parse::<Variant>().unwrap_err();
        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("metadata"))
        );

        let err = "`Not Reported`
        "
        .parse::<Variant>()
        .unwrap_err();
        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("metadata"))
        );

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_a_variant_with_a_missing_description(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "`Not Reported`
        
        * **Hello**: World"
            .parse::<Variant>()
            .unwrap_err();
        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("description"))
        );

        let err = "`Not Reported`
        
        * **Hello**: World
        "
        .parse::<Variant>()
        .unwrap_err();
        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("description"))
        );

        Ok(())
    }

    #[test]
    fn it_fails_to_parse_an_invalid_permissible_value(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "Not Reported
        
        * **Hello**: World
        
        A description."
            .parse::<Variant>()
            .unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidPermissibleValueFormat(String::from("Not Reported"))
        );

        let err = "`Not Reported`
        
        * **Hello**: World
        "
        .parse::<Variant>()
        .unwrap_err();
        assert_eq!(
            err,
            ParseError::IteratorEndedEarly(String::from("description"))
        );

        Ok(())
    }
}
