use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use crate::Namespace;

/// The primary name and namespace for a file within the source server.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::file::Identifier)]
pub struct Identifier(cde::v1::file::Identifier);

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let identifier = Identifier::new(&namespace, "File001.txt");
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("File001.txt"));
    /// ```
    pub fn new(namespace: &Namespace, name: impl Into<String>) -> Self {
        Self(cde::v1::file::Identifier::new(
            namespace.name().to_string(),
            name.into(),
        ))
    }

    /// Gets the namespace for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    pub fn namespace(&self) -> &str {
        self.0.namespace()
    }

    /// Gets the name for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let identifier = Identifier::new(&namespace, "Name");
    /// assert_eq!(identifier.name(), &String::from("Name"));
    /// ```
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// Consumes `self` to get the inner [`cde::v1::file::Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let identifier = Identifier::new(&namespace, "Name");
    /// let inner = identifier.into_inner();
    ///
    /// assert_eq!(inner.namespace(), String::from("organization"));
    /// assert_eq!(inner.name(), String::from("Name"));
    /// ```
    pub fn into_inner(self) -> cde::v1::file::Identifier {
        self.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
