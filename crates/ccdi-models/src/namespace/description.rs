use std::ops::Deref;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The maximum number of characters for a description.
///
/// NOTE: if you change this number, be sure to also update the number in the
/// docstring for [`Description`] so that it is reflected in the OpenAPI
/// documentation as well.
const MAX_CHARACTERS: usize = 2048;

#[derive(Debug)]
pub enum Error {
    /// An empty description was provided.
    Empty,

    /// A description with more than [`MAX_CHARACTERS`] characters was provided.
    TooLong(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Empty => todo!(),
            Error::TooLong(len) => write!(
                f,
                "too long: {} chars exceeds maximum of {} chars",
                len, MAX_CHARACTERS
            ),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

/// A description of a namespace.
///
/// This description cannot exceed 2048 characters.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[schema(
    as = models::namespace::Description,
    example = "A namespace owned by Example Organization."
)]
pub struct Description(String);

impl Description {
    /// Creates a new [`Description`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    ///
    /// let description = Description::try_new("Here is a description").unwrap();
    /// ```
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(Error::Empty);
        }

        if value.len() > MAX_CHARACTERS {
            return Err(Error::TooLong(value.len()));
        }

        Ok(Self(value))
    }
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Description {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl FromStr for Description {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_new(s.to_string())
    }
}

impl Deref for Description {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_successfully_creates_a_description() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let description = "Hello, world!".parse::<Description>()?;
        assert_eq!(description.to_string(), "Hello, world!");

        let description = Description::try_new("Hello, world!")?;
        assert_eq!(description.to_string(), "Hello, world!");

        Ok(())
    }

    #[test]
    fn it_handles_length_issues_correctly() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let description = "a".repeat(MAX_CHARACTERS).parse::<Description>()?;
        assert_eq!(description.to_string(), "a".repeat(MAX_CHARACTERS));

        let err = "a"
            .repeat(MAX_CHARACTERS + 1)
            .parse::<Description>()
            .unwrap_err();
        assert!(matches!(err, Error::TooLong(_)));

        Ok(())
    }

    #[test]
    fn it_fails_to_create_an_empty_description(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "".parse::<Description>().unwrap_err();
        assert!(matches!(err, Error::Empty));

        Ok(())
    }
}
