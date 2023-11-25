use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Namespace;

/// A name and namespace for a [`File`](crate::File).
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::file::Identifier)]
pub struct Identifier {
    /// The namespace of the identifier.
    #[schema(example = "organization")]
    namespace: String,

    /// The name of the file.
    #[schema(example = "File001.txt")]
    name: String,
}

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
    /// let identifier = Identifier::new(&namespace, "Sample");
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
    /// assert_eq!(identifier.name(), &String::from("Sample"));
    /// ```
    pub fn new(namespace: &Namespace, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.name().to_string(),
            name: name.into(),
        }
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
    ///
    /// let identifier = Identifier::new(&namespace, "Name");
    /// assert_eq!(identifier.namespace(), "organization");
    /// ```
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
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
    /// assert_eq!(identifier.name(), "Name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
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
