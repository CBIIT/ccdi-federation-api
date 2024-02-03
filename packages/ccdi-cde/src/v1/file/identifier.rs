//! An identifier for a file.

use introspect::Introspect;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

/// A parse error related to an [`Identifier`].
#[derive(Debug)]
pub enum ParseError {
    /// An invalid format was encountered.
    InvalidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(value) => write!(f, "invalid format: {value}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An error related to an [`Identifier`].
#[derive(Debug)]
pub enum Error {
    /// A parse error.
    ParseError(ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(err) => write!(f, "parse error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// **`caDSR CDE 11284037 v1.00`**
///
/// This metadata element is defined by the caDSR as "The literal label for an
/// electronic data file.". No permissible values are defined for this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=11284037%20and%20ver_nr=1>
#[derive(
    Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema, Introspect,
)]
#[schema(as = cde::v1::file::Identifier)]
pub struct Identifier {
    /// The namespace of the identifier.
    #[schema(example = "organization")]
    namespace: String,

    /// The name of the identifier.
    #[schema(example = "File001.txt")]
    name: String,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Identifier;
    ///
    /// let identifier = Identifier::new("organization", "File001.txt");
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("File001.txt"));
    /// ```
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Gets the namespace for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:File001.txt", ":")?;
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    /// Gets the name for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:File001.txt", ":")?;
    /// assert_eq!(identifier.name(), &String::from("File001.txt"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Parses an [`Identifier`] from a [`&str`](str) using the provided
    /// delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::file::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:File001.txt", ":")?;
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("File001.txt"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse(s: &str, separator: &str) -> Result<Self, Error> {
        let parts = s.split(separator).collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(Error::ParseError(ParseError::InvalidFormat(s.to_owned())));
        }

        let mut parts = parts.iter();

        // SAFETY: we just checked that two parts must exist. Since we have not
        // consumed any items from the iterator, these two unwraps will always
        // succeed.
        let namespace = parts.next().unwrap().to_string();
        let name = parts.next().unwrap().to_string();

        Ok(Self { namespace, name })
    }
}

impl CDE for Identifier {}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ namespace: {}, name: {} }}",
            self.namespace, self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::file::Identifier;

    #[test]
    fn it_displays_correctly() {
        let identifier = Identifier::new("organization", "File001.txt");
        assert_eq!(
            identifier.to_string(),
            "{ namespace: organization, name: File001.txt }"
        );
    }
}
