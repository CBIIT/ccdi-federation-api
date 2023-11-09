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
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    as = models::namespace::Description,
    example = "A namespace owned by Example Organization."
)]
pub struct Description(String);

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Description {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        parse(value)
    }
}

impl FromStr for Description {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse(s.to_string())
    }
}

pub fn parse(value: String) -> Result<Description> {
    if value.is_empty() {
        return Err(Error::Empty);
    }

    if value.len() > MAX_CHARACTERS {
        return Err(Error::TooLong(value.len()));
    }

    Ok(Description(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_successfully_creates_a_description() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let description = "Hello, world!".parse::<Description>()?;
        assert_eq!(description.to_string(), "Hello, world!");

        let description = Description::try_from(String::from("Hello, world!"))?;
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
