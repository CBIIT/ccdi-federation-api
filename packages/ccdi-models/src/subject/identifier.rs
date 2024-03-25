//! Identifiers for subjects.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_cde as cde;

use crate::namespace;

pub mod linked;
pub mod referenced;
pub mod unlinked;

/// An identifier for a [`Subject`](crate::Subject).
///
/// [`Identifiers`](Identifier) serve two main purposes:
///
/// 1. They represent the primary identifier for a [`Subject`](crate::Subject).
/// 2. They extended when referenced as [linked identifiers](linked::Identifier).
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::subject::Identifier)]
pub struct Identifier {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

    // The name of the identifier.
    #[schema(example = "SubjectName001")]
    name: cde::v1::subject::Name,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::subject::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::subject::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    ///
    /// assert_eq!(
    ///     subject_id.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(subject_id.namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(subject_id.name().as_str(), "SubjectName001");
    /// ```
    pub fn new(namespace: namespace::Identifier, name: impl Into<cde::v1::subject::Name>) -> Self {
        let name = name.into();

        Self { name, namespace }
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
    /// use models::subject::Identifier;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         // SAFETY: this is manually crafted to succeed every time.
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    /// );
    ///
    /// let name = Identifier::new(namespace.id().clone(), "Name");
    /// assert_eq!(name.namespace(), namespace.id());
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
    /// use models::subject::Identifier;
    /// use models::Namespace;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// let namespace = Namespace::new(
    ///     namespace::Identifier::new(
    ///         organization.id().clone(),
    ///         // SAFETY: this is manually crafted to succeed every time.
    ///         "ExampleNamespace"
    ///             .parse::<namespace::identifier::Name>()
    ///             .unwrap(),
    ///     ),
    ///     "support@example.com",
    ///     None,
    /// );
    ///
    /// let name = Identifier::new(namespace.id().clone(), "Name");
    /// assert_eq!(name.name().as_str(), "Name");
    /// ```
    pub fn name(&self) -> &cde::v1::subject::Name {
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
