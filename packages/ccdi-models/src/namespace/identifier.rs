//! Namespace identifiers.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod name;

pub use name::Name;

use crate::organization;

/// An identifier for a namespace.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToSchema)]
#[schema(as = models::namespace::Identifier)]
pub struct Identifier {
    /// The organization that owns this namespace.
    ///
    /// The identifier **must** conform to
    /// [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
    /// matching the pattern `^[a-z0-9-]+$`. Any identifier that does not match this
    /// pattern should be considered invalid by clients.
    ///
    /// **Note:** this field is asserted by the source server, but it is not
    /// guaranteed to be authoritative across the federation (due to the
    /// decentralized nature of organization and namespace allocation).
    ///
    /// **Note**: the regex for this field does not allow for any spaces because it is
    /// anticipated that the field will be displayable as a repository (e.g.,
    /// `example-organization/ExampleNamespace`).
    #[schema(example = "example-organization", value_type = models::organization::Identifier)]
    organization: organization::Identifier,

    /// The name of this namespace.
    ///
    /// Typically, this is going to represent a particular dataset within the source server.
    /// The name **must** conform to the pattern `^[A-Za-z0-9-]+$`. Any name that does not
    /// match this pattern should be considered invalid by clients.
    ///
    /// NOTE: the regex for this field does not allow for any spaces because it is
    /// anticipated that the field will be displayable as a repository (e.g.,
    /// `example-organization/ExampleNamespace`).
    #[schema(example = "ExampleNamespace", value_type = models::organization::Name)]
    name: Name,
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
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let identifier = Identifier::new(
    ///     organization.id().clone(),
    ///     "ExampleNamespace"
    ///         .parse::<namespace::identifier::Name>()
    ///         .unwrap(),
    /// );
    /// ```
    pub fn new(organization: organization::Identifier, name: impl Into<Name>) -> Self {
        let name = name.into();

        Self { organization, name }
    }

    /// Gets the organization for this [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let identifier = Identifier::new(
    ///     organization.id().clone(),
    ///     "ExampleNamespace"
    ///         .parse::<namespace::identifier::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// assert_eq!(identifier.organization().as_str(), "example-organization");
    /// ```
    pub fn organization(&self) -> &organization::Identifier {
        &self.organization
    }

    /// Gets the name of this [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// let identifier = Identifier::new(
    ///     organization.id().clone(),
    ///     "ExampleNamespace"
    ///         .parse::<namespace::identifier::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// assert_eq!(identifier.name().as_str(), "ExampleNamespace");
    /// ```
    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ organization: {}, name: {} }}",
            self.organization.as_str(),
            self.name.as_str()
        )
    }
}
