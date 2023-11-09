use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^[a-z0-9-]+$").unwrap();
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

/// The name of a namespace.
///
/// The name **must** conform to
/// [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
/// matching the pattern `^[a-z0-9-]+$`. Any name that does not match this
/// pattern should be considered invalid by clients.
///
/// **Note:** this field is asserted by the source server, but it is not
/// guaranteed to be authoritative across the federation (due to the
/// decentralized nature of namespace allocation).
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::namespace::Name)]
pub struct Name(String);

impl std::ops::Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !PATTERN.is_match(&value) {
            return Err(Error::Parse(ParseError::InvalidFormat(format!(
                "name must conform to the pattern {}",
                PATTERN.as_str()
            ))));
        }

        Ok(Name(value))
    }
}
