use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

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

/// The primary name and namespace for a sample used within the source server.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::sample::Identifier)]
pub struct Identifier {
    /// The namespace of the identifier.
    #[schema(example = "organization")]
    namespace: String,

    /// The name of the identifier.
    #[schema(example = "SampleName001")]
    name: String,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use models::sample::Identifier;
    ///
    /// let identifier = Identifier::new("organization", "Sample");
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("Sample"));
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
    /// use ccdi_models as models;
    /// use models::sample::Identifier;
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
    /// use ccdi_models as models;
    /// use models::sample::Identifier;
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
    /// use ccdi_models as models;
    /// use models::sample::Identifier;
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

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ namespace: {}, name: {} }}",
            self.namespace, self.name
        )
    }
}
