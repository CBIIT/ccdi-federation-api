use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Namespace;

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
    ///
    /// use models::sample::Identifier;
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
    /// use models::sample::Identifier;
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
    /// assert_eq!(identifier.namespace(), &String::from("organization"));
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
    /// use models::sample::Identifier;
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
