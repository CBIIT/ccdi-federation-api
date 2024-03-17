//! Identifiers for samples.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod linked;
pub mod referenced;
pub mod unlinked;

use crate::namespace;

/// An identifier for a [`Sample`](crate::Sample).
///
/// [`Identifiers`](Identifier) serve two main purposes:
///
/// 1. They represent the primary identifier for a [`Sample`](crate::Sample).
/// 2. They extended when referenced as [linked identifiers](linked::Identifier).
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::sample::Identifier)]
pub struct Identifier {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

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
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
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
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// assert_eq!(
    ///     sample_id.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(sample_id.namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(sample_id.name(), "SampleName001");
    /// ```
    pub fn new(namespace: namespace::Identifier, name: impl Into<String>) -> Self {
        Self {
            namespace,
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
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
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
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// assert_eq!(
    ///     sample_id.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(sample_id.namespace().name().as_str(), "ExampleNamespace");
    /// ```
    pub fn namespace(&self) -> &namespace::Identifier {
        &self.namespace
    }

    /// Gets the name for the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
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
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    /// assert_eq!(sample_id.name(), "SampleName001");
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
