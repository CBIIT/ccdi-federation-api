use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^[A-Za-z0-9-]+$").unwrap();
}

#[derive(Debug)]
pub enum ParseError {
    /// Attempted to create a name with an invalid format.
    InvalidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(reason) => write!(f, "invalid format: {reason}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An error related to a [`Name`].
#[derive(Debug)]
pub enum Error {
    /// A parse error.
    Parse(ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(err) => write!(f, "parse error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The name of a namespace.
///
/// Typically, this is going to represent a particular dataset within the source server.
/// The name **must** conform to the pattern `^[A-Za-z0-9-]+$`. Any name that does not
/// match this pattern should be considered invalid by clients.
///
/// NOTE: the regex for this field does not allow for any spaces because it is
/// anticipated that the field will be displayable as a repository (e.g.,
/// `example-organization/ExampleNamespace`).
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::namespace::identifier::Name)]
pub struct Name(String);

impl Name {
    /// Attempts to create a new [`Name`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::identifier::Name;
    ///
    /// let name = Name::try_new("ExampleNamespace").unwrap();
    /// ```
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if !PATTERN.is_match(&value) {
            return Err(Error::Parse(ParseError::InvalidFormat(format!(
                "name must conform to the pattern {}",
                PATTERN.as_str()
            ))));
        }

        Ok(Name(value))
    }
}

impl std::ops::Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allows_valid_patterns() {
        "hello-world".parse::<Name>().unwrap();
        "testing".parse::<Name>().unwrap();
    }

    #[test]
    fn it_does_not_allow_invalid_patterns() {
        "".parse::<Name>().unwrap_err();
        "Hello World".parse::<Name>().unwrap_err();
        "á".parse::<Name>().unwrap_err();
    }
}
