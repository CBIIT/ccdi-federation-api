use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The maximum number of characters for an organization name.
///
/// NOTE: if you change this number, be sure to also update the number
/// in the docstring for [`super::Organization`] so that it is
/// reflected in the OpenAPI documentation as well.
const MAX_CHARACTERS: usize = 256;

#[derive(Debug)]
pub enum Error {
    /// An empty organization name was provided.
    Empty,

    /// A organization name with more than [`MAX_CHARACTERS`] characters was provided.
    TooLong(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Empty => todo!(),
            Error::TooLong(len) => write!(
                f,
                "too long: {len} chars exceeds maximum of {MAX_CHARACTERS} chars"
            ),
        }
    }
}

impl std::error::Error for Error {}
type Result<T> = std::result::Result<T, Error>;

/// The proper name of the organization as it should be displayed by clients.
///
/// This name name cannot exceed 256 characters.
///
/// This field is intended to be the proper name of the organization that mints
/// identifiers within a given namespace. That said, we have intentionally not
/// required that this be an organization specifically, as there may be exceptions
/// to this guideline. We recommend that you use an organization name here if you
/// can, but you may put whatever value is appropriate to describe the owner of the
/// namespace.
///
/// It is recommended that you use title case for this field, though that is not
/// required.
///
/// **Note:** this field is asserted by the source server, but it is not guaranteed
/// to be authoritative across the federation (due to the decentralized nature of
/// organization and namespace allocation).
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::organization::Name, example = "Example Organization")]
pub struct Name(String);

impl Name {
    /// Attempts to create a new [`Name`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::organization::Name;
    ///
    /// let name = Name::try_new("example-organization").unwrap();
    /// ```
    pub fn try_new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        parse(value)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse(s.to_string())
    }
}

pub fn parse(value: String) -> Result<Name> {
    if value.is_empty() {
        return Err(Error::Empty);
    }

    if value.len() > MAX_CHARACTERS {
        return Err(Error::TooLong(value.len()));
    }

    Ok(Name(value))
}

impl std::ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_successfully_creates_a_organization_name(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let organization_name = "Children's Hospital of Philadelphia".parse::<Name>()?;
        assert_eq!(
            organization_name.to_string(),
            "Children's Hospital of Philadelphia"
        );

        let organization_name = Name::try_new(String::from("Children's Hospital of Philadelphia"))?;
        assert_eq!(
            organization_name.to_string(),
            "Children's Hospital of Philadelphia"
        );

        Ok(())
    }

    #[test]
    fn it_handles_length_issues_correctly() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let organization_name = "a".repeat(MAX_CHARACTERS).parse::<Name>()?;
        assert_eq!(organization_name.to_string(), "a".repeat(MAX_CHARACTERS));

        let err = "a".repeat(MAX_CHARACTERS + 1).parse::<Name>().unwrap_err();
        assert!(matches!(err, Error::TooLong(_)));

        Ok(())
    }

    #[test]
    fn it_fails_to_create_an_empty_organization_name(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let err = "".parse::<Name>().unwrap_err();
        assert!(matches!(err, Error::Empty));

        Ok(())
    }
}
