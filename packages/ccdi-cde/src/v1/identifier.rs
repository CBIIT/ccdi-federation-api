use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::CDE;

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

/// **caDSR CDE 6380049 v1.00**
///
/// This metadata element is defined by the caDSR as "A unique subject
/// identifier within a site and a study.". No permissible values are defined
/// for this CDE.
///
/// Link:
/// <https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6380049%20and%20ver_nr=1>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = cde::v1::Identifier)]
pub struct Identifier
where
    Self: CDE,
{
    /// The namespace of the identifier.
    #[schema(example = "organization")]
    namespace: String,

    /// The name of the identifier.
    #[schema(example = "SubjectName001")]
    name: String,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::Identifier;
    ///
    /// let identifier = Identifier::new("organization", "Name");
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("Name"));
    /// ```
    pub fn new<S: Into<String>>(namespace: S, name: S) -> Self {
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
    /// use cde::v1::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:Name", ":")?;
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn namespace(&self) -> &String {
        &self.namespace
    }

    /// Gets the name for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:Name", ":")?;
    /// assert_eq!(identifier.name(), &String::from("Name"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Parses an [`Identifier`] from a [`&str`](str) using the provided
    /// delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use cde::v1::Identifier;
    ///
    /// let identifier = Identifier::parse("organization:Name", ":")?;
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("Name"));
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

impl crate::Standard for Identifier {
    fn standard() -> &'static str {
        "caDSR CDE 6380049 v1.00"
    }
}

impl crate::Url for Identifier {
    fn url() -> &'static str {
        "https://cadsr.cancer.gov/onedata/dmdirect/NIH/NCI/CO/CDEDD?filter=CDEDD.ITEM_ID=6380049%20and%20ver_nr=1"
    }
}

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
    use crate::v1::Identifier;

    #[test]
    fn it_displays_correctly() {
        let identifier = Identifier::new("organization", "Name");
        assert_eq!(
            identifier.to_string(),
            "{ namespace: organization, name: Name }"
        );
    }
}
