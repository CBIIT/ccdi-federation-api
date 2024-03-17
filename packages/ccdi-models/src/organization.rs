//! Organizations within a namespace.

use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod identifier;
mod name;

pub use identifier::Identifier;
pub use name::Name;

/// An organization.
///
/// Organizations own [`Namespaces`](super::Namespace) within a source server.
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::Organization)]
pub struct Organization {
    /// The identifier of this organization.
    ///
    /// The name **must** conform to
    /// [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
    /// matching the pattern `^[a-z0-9-]+$`. Any name that does not match this
    /// pattern should be considered invalid by clients.
    ///
    /// **Note**: the regex for this field does not allow for any spaces because it is
    /// anticipated that the field will be displayable as a repository (e.g.,
    /// `example-organization/ExampleNamespace`).
    #[schema(
        value_type = models::organization::Identifier,
        example = "example-organization"
    )]
    identifier: Identifier,

    /// The proper name of the organization as it should be displayed by clients.
    ///
    /// This field is intended to be the proper name of the organization that mints
    /// identifiers within a given namespace. That said, we have intentionally not
    /// required that this be an organization specifically, as there may be exceptions
    /// to this guideline. We recommend that you use an organization name here if you
    /// can, but you may put whatever value is appropriate to describe the owner of the
    /// namespace.
    ///
    /// It is recommended that you use title case for this field, though that is not
    /// required.
    ///
    /// **Note:** this field is asserted by the source server, but it is not guaranteed
    /// to be authoritative across the federation (due to the decentralized nature of
    /// organization and namespace allocation).
    #[schema(example = "Example Organization")]
    name: String,
}

impl Organization {
    /// Creates a new [`Organization`].
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::organization::Identifier;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization".parse::<Identifier>().unwrap(),
    ///     "Example Organization",
    /// );
    /// ```
    pub fn new(identifier: impl Into<Identifier>, name: impl Into<String>) -> Self {
        let identifier = identifier.into();
        let name = name.into();

        Self { identifier, name }
    }

    /// Gets the identifier of the [`Organization`] by reference.
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::organization::Identifier;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization".parse::<Identifier>().unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// assert_eq!(organization.id().as_str(), "example-organization");
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.identifier
    }

    /// Gets the name of the [`Organization`] by reference.
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::organization::Identifier;
    /// use models::Organization;
    ///
    /// let organization = Organization::new(
    ///     "example-organization".parse::<Identifier>().unwrap(),
    ///     "Example Organization",
    /// );
    ///
    /// assert_eq!(organization.name(), "Example Organization");
    /// ```
    pub fn name(&self) -> &str {
        self.name.deref()
    }
}
