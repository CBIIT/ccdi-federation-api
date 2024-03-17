use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use crate::namespace;

/// The primary name and namespace for a file within the source server.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::file::Identifier)]
pub struct Identifier {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

    name: cde::v1::file::Name,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let identifier = Identifier::new(
    ///     namespace.id().clone(),
    ///     cde::v1::file::Name::new("File001.txt"),
    /// );
    ///
    /// assert_eq!(
    ///     identifier.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(identifier.namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(identifier.name().as_str(), "File001.txt");
    /// ```
    pub fn new(namespace: namespace::Identifier, name: cde::v1::file::Name) -> Self {
        Self { name, namespace }
    }

    /// Gets the namespace for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let identifier = Identifier::new(
    ///     namespace.id().clone(),
    ///     cde::v1::file::Name::new("File001.txt"),
    /// );
    ///
    /// assert_eq!(
    ///     identifier.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(identifier.namespace().name().as_str(), "ExampleNamespace");
    /// ```
    pub fn namespace(&self) -> &namespace::Identifier {
        &self.namespace
    }

    /// Gets the name for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let identifier = Identifier::new(
    ///     namespace.id().clone(),
    ///     cde::v1::file::Name::new("File001.txt"),
    /// );
    ///
    /// assert_eq!(identifier.name().as_str(), "File001.txt");
    /// ```
    pub fn name(&self) -> &cde::v1::file::Name {
        &self.name
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
