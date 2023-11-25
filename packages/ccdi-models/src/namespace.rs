//! Responses related to describing namespaces.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod description;
mod name;

pub use description::Description;
pub use name::Name;

/// An error related to a [`Namespace`].
#[derive(Debug)]
pub enum Error {
    /// A name error.
    Name(name::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Name(err) => write!(f, "name error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A response for describing metadata fields for a subject.
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::Namespace)]
pub struct Namespace {
    /// The name of this namespace.
    ///
    /// The name **must** conform to
    /// [`kebab-casing`](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case),
    /// matching the pattern `^[a-z0-9-]+$`. Any name that does not match this
    /// pattern should be considered invalid by clients.
    #[schema(
        value_type = models::namespace::Name,
        example = "organization"
    )]
    name: Name,

    /// A free-text string describing the owner of the namespace.
    ///
    /// This field is intended to be the proper name of the organization that
    /// mints identifiers within a given namespace. That said, we have
    /// intentionally not required this restriction, as there may be exceptions
    /// to this guideline. We recommend that you use an organization name here
    /// if you can, but you may put whatever value is appropriate to describe
    /// the owner of the namespace.
    ///
    /// It is recommended that you use title case for this field, though that is
    /// not strictly required.
    ///
    /// **Note:** this field is asserted by the source server, but it is not
    /// guaranteed to be authoritative across the federation (due to the
    /// decentralized nature of namespace allocation).
    #[schema(example = "Example Organization")]
    owner: String,

    /// A support email address for entities contained within the namespace.
    ///
    /// This field is required to be a valid email address (both in format and
    /// in terms of the email address being actively monitored).
    #[schema(example = "support@example.com")]
    contact_email: String,

    /// If available, a description for the namespace.
    ///
    /// This is free-text field with a maximum length in characters (please see
    /// the definition of [`Description`] for maximum number of characters).
    #[schema(
        value_type = models::namespace::Description,
        example = "A namespace owned by Example Organization."
    )]
    description: Option<Description>,
}

impl Namespace {
    /// Creates a new namespace [`Namespace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     Some("Hello, world!".parse::<Description>().unwrap()),
    /// )
    /// .unwrap();
    /// ```
    pub fn try_new(
        name: impl Into<String>,
        owner: impl Into<String>,
        contact_email: impl Into<String>,
        description: Option<Description>,
    ) -> Result<Self> {
        let name = Name::try_from(name.into()).map_err(Error::Name)?;

        Ok(Self {
            name,
            owner: owner.into(),
            contact_email: contact_email.into(),
            description,
        })
    }

    /// Gets the name of the [`Namespace`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     Some("Hello, world!".parse::<Description>().unwrap()),
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(namespace.name(), "organization");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the owner of the [`Namespace`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     Some("Hello, world!".parse::<Description>().unwrap()),
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(namespace.owner(), "Example Organization");
    /// ```
    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    /// Gets the contact email of the [`Namespace`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     Some("Hello, world!".parse::<Description>().unwrap()),
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(namespace.contact_email(), "support@example.com");
    /// ```
    pub fn contact_email(&self) -> &str {
        self.contact_email.as_str()
    }

    /// Gets the description of the [`Namespace`] by reference (if it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::namespace::Description;
    /// use models::Namespace;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     Some("Hello, world!".parse::<Description>().unwrap()),
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(
    ///     namespace.description().unwrap().to_string().as_str(),
    ///     "Hello, world!"
    /// );
    /// ```
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
}
