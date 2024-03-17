//! Organization identifiers.

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^[a-z0-9-]+$").unwrap();
}

/// An error when parsing an [`Identifier`].
#[derive(Debug)]
pub enum ParseError {
    /// Attempted to create a identifier with an invalid format.
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

/// An error related to a [`Identifier`].
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

/// The identifier of an organization.
///
/// The identifier **must** conform to
/// [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
/// matching the pattern `^[a-z0-9-]+$`. Any identifier that does not match this
/// pattern should be considered invalid by clients.
///
/// **Note:** this field is asserted by the source server, but it is not
/// guaranteed to be authoritative across the federation (due to the
/// decentralized nature of organization and namespace allocation).
///
/// **Note**: the regex for this field does not allow for any spaces because it is
/// anticipated that the field will be displayable as a repository (e.g.,
/// `example-organization/ExampleNamespace`).
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::organization::Identifier)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::organization::Identifier;
    ///
    /// let identifier = Identifier::try_new("example-organization").unwrap();
    /// ```
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if !PATTERN.is_match(&value) {
            return Err(Error::Parse(ParseError::InvalidFormat(format!(
                "identifier must conform to the pattern {}",
                PATTERN.as_str()
            ))));
        }

        Ok(Identifier(value))
    }
}

impl std::ops::Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::try_new(s)
    }
}
