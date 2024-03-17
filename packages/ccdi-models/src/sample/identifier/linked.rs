//! Linked identifiers for [`Samples`](crate::Sample).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Url;

/// A linked identifier for a [`Sample`](crate::Sample).
///
/// Linked identifiers are identifiers that are able to be linked back to servers within
/// the federated ecosystem (i.e., the server that owns this identifier within the
/// ecosystem is known).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::sample::identifier::linked::Identifier)]
pub struct Identifier {
    /// The inner sample identifier.
    #[serde(flatten)]
    #[schema(value_type = models::sample::Identifier)]
    inner: super::Identifier,

    /// The server from whence this identifier is sourced.
    #[schema(
        value_type = models::Url,
        example = "https://ccdi.example.com/api/v0"
    )]
    server: Url,
}

impl Identifier {
    /// Creates a new [`Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
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
    /// let linked_id = models::sample::identifier::linked::Identifier::new(
    ///     sample_id,
    ///     "https://ccdi.example.com/api/v0".parse::<Url>().unwrap(),
    /// );
    ///
    /// assert_eq!(
    ///     linked_id.inner().namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(
    ///     linked_id.inner().namespace().name().as_str(),
    ///     "ExampleNamespace"
    /// );
    /// assert_eq!(linked_id.inner().name(), "SampleName001");
    /// assert_eq!(
    ///     linked_id.server().as_str(),
    ///     "https://ccdi.example.com/api/v0"
    /// );
    /// ```
    pub fn new(inner: super::Identifier, server: Url) -> Self {
        Self { inner, server }
    }

    /// Gets the inner [`super::Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
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
    /// let linked_id = models::sample::identifier::linked::Identifier::new(
    ///     sample_id,
    ///     "https://ccdi.example.com/api/v0".parse::<Url>().unwrap(),
    /// );
    ///
    /// assert_eq!(
    ///     linked_id.inner().namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(
    ///     linked_id.inner().namespace().name().as_str(),
    ///     "ExampleNamespace"
    /// );
    /// assert_eq!(linked_id.inner().name(), "SampleName001");
    /// ```
    pub fn inner(&self) -> &super::Identifier {
        &self.inner
    }

    /// Consumes `self` and returns the inner [`super::Identifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
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
    /// let linked_id = models::sample::identifier::linked::Identifier::new(
    ///     sample_id,
    ///     "https://ccdi.example.com/api/v0".parse::<Url>().unwrap(),
    /// );
    ///
    /// let sample_id = linked_id.into_inner();
    ///
    /// assert_eq!(
    ///     sample_id.namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(sample_id.namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(sample_id.name(), "SampleName001");
    /// ```
    pub fn into_inner(self) -> super::Identifier {
        self.inner
    }

    /// Gets the source server from the [`Identifier`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::metadata::field::unowned::sample::Identifier;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
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
    /// let linked_id = models::sample::identifier::linked::Identifier::new(
    ///     sample_id,
    ///     "https://ccdi.example.com/api/v0".parse::<Url>().unwrap(),
    /// );
    ///
    /// assert_eq!(
    ///     linked_id.server().as_str(),
    ///     "https://ccdi.example.com/api/v0"
    /// );
    /// ```
    pub fn server(&self) -> &Url {
        &self.server
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ inner: {}, server: {} }}",
            self.inner,
            self.server.as_str()
        )
    }
}
