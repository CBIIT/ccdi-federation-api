//! Parsing the members of `struct`s ("fields") as common data elements.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Attempted to parse a field with no documentation.
    Empty,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "cannot parse field with no documentation"),
        }
    }
}

impl std::error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

/// A parsed field of a `struct` that describes a common data element.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
pub struct Field(String);

impl Field {
    /// Gets the description for the [`Field`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    ///
    /// use cde::parse::cde::member::Field;
    ///
    /// let member = "The namespace of the identifier.".parse::<Field>()?;
    ///
    /// assert_eq!(member.description(), "The namespace of the identifier.");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn description(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }

        Ok(Field::from(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fails_to_parse_a_variant_with_no_documentation(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "".parse::<Field>().unwrap_err();
        assert_eq!(err, ParseError::Empty);

        Ok(())
    }
}
