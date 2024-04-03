//! Responses related to describing namespaces.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod description;
pub mod identifier;

pub use description::Description;
pub use identifier::Identifier;

/// A namespace.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Namespace)]
pub struct Namespace {
    /// The identifier for this namespace.
    #[schema(value_type = models::namespace::Identifier)]
    id: Identifier,

    /// If available, a description for the namespace.
    ///
    /// This is free-text field with a maximum length in characters (please see
    /// the definition of [`Description`] for maximum number of characters).
    #[schema(
        value_type = models::namespace::Description,
        example = "A namespace owned by Example Organization."
    )]
    description: Option<Description>,

    /// A support email address for entities contained within the namespace.
    ///
    /// This field is required to be a valid email address (both in format and
    /// in terms of the email address being actively monitored).
    #[schema(example = "support@example.com")]
    contact_email: String,
}

impl Namespace {
    /// Creates a new namespace [`Namespace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::organization;
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
    ///     Some("Hello, world!".parse::<namespace::Description>().unwrap()),
    /// );
    /// ```
    pub fn new(
        id: impl Into<Identifier>,
        contact_email: impl Into<String>,
        description: Option<Description>,
    ) -> Self {
        Self {
            id: id.into(),
            contact_email: contact_email.into(),
            description,
        }
    }

    /// Gets the identifier of the [`Namespace`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::organization;
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
    ///     Some("Hello, world!".parse::<namespace::Description>().unwrap()),
    /// );
    ///
    /// assert_eq!(
    ///     namespace.id().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(namespace.id().name().as_str(), "ExampleNamespace");
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the description of the [`Namespace`] by reference (if it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::organization;
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
    ///     Some("Hello, world!".parse::<namespace::Description>().unwrap()),
    /// );
    ///
    /// assert_eq!(namespace.description().unwrap().as_str(), "Hello, world!");
    /// ```
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    /// Gets the contact email of the [`Namespace`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace;
    /// use models::organization;
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
    ///     Some("Hello, world!".parse::<namespace::Description>().unwrap()),
    /// );
    ///
    /// assert_eq!(namespace.contact_email(), "support@example.com");
    /// ```
    pub fn contact_email(&self) -> &str {
        self.contact_email.as_str()
    }
}
