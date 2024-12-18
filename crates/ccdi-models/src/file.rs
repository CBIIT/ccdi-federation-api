//! Representations of files.

use nonempty::NonEmpty;
use rand::thread_rng;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod identifier;
pub mod metadata;

pub use identifier::Identifier;
pub use metadata::Metadata;

use crate::gateway;
use crate::gateway::AnonymousOrReference;
use crate::gateway::Link;
use crate::Entity;
use crate::Url;

/// A file.
///
/// **Note:** the `samples` key **must** include only identifiers for
/// [`Samples`](super::Sample) that both (a) are listed in the
/// [`Sample`](super::Sample) index endpoint (`/sample`) and (b) are able to be
/// shown with the [`Sample`](super::Sample) show endpoint
/// (`/sample/{namespace}/{name}`).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::File)]
pub struct File {
    /// The primary identifier used by the site for this [`File`].
    ///
    /// This namespace pointed to by this identifier must also **ALWAYS** be
    /// included in results provided by the `/namespace` endpoint (and the
    /// subsequent `/namespace/<name>` endpoint). Failure to include the
    /// namespace in the results at that endpoint signifies non-compliance with
    /// the API.
    #[schema(value_type = models::file::Identifier)]
    id: Identifier,

    /// One or more samples that are associated with this [`File`] referred to
    /// by their identifier(s).
    ///
    /// Each file must be associated with at least one
    /// [`Sample`](super::Sample)—files that are not associated with any sample
    /// are disallowed. Files may be associated with as many samples as is
    /// necessary.
    ///
    /// **Note:** each identifier **must** match a [`Sample`](super::Sample)
    /// that both (a) is listed in the [`Sample`](super::Sample) index endpoint
    /// (`/sample`) and (b) is able to be shown with the
    /// [`Sample`](super::Sample) show endpoint (`/sample/{namespace}/{name}`).
    #[schema(value_type = Vec<models::sample::Identifier>)]
    samples: NonEmpty<crate::sample::Identifier>,

    /// One or more [gateways](AnonymousOrReference) through which this file can
    /// be accessed.
    ///
    /// Gateways can be either [anonymous](AnonymousOrReference::Anonymous)
    /// ([gateways](crate::Gateway) with no name) or a
    /// [refererence](AnonymousOrReference::Reference) to a [named
    /// gateway](gateway::Named) ([gateways](crate::Gateway) with a name).
    ///
    /// **Anonymous** gateways are intended to be embedded directly within a
    /// returned file in the `/file` response object. They have no name and are
    /// only referred to by the file within which they are embedded.
    ///
    /// **Named** gateways, on the other hand, are included in the `gateways`
    /// key in the `/file` response object and referred to by name within a
    /// returned file in the `/file` response object. They are intended to be
    /// used when more than one file references the same gateway. This mechanism
    /// is available to ensure that the gateway object does not need to be
    /// duplicated multiple times in the response in these cases.
    ///
    /// This field can contain multiple gateways to support scenarios where a
    /// file is available through more than one mechanism. We expect that only
    /// one gateway will be returned in most responses.
    ///
    /// **Note:** a file must have at least one gateway. If the file has no
    /// gateway, then it should not be returned as part of this API.
    #[schema(
        value_type = Vec<models::gateway::AnonymousOrReference>,
        required = false,
        nullable = false,
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<NonEmpty<gateway::AnonymousOrReference>>,

    /// Harmonized metadata associated with this [`File`].
    #[schema(
        value_type = Option<models::file::Metadata>,
        nullable = true
    )]
    metadata: Option<Metadata>,
}

impl File {
    /// Creates a new [`File`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        id: Identifier,
        samples: NonEmpty<crate::sample::Identifier>,
        gateways: Option<NonEmpty<gateway::AnonymousOrReference>>,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            id,
            samples,
            gateways,
            metadata,
        }
    }

    /// Gets the identifier for this [`File`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// assert_eq!(
    ///     file.id().namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(file.id().namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(file.id().name().as_str(), "Foo.txt");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the identifier(s) for the [`Sample(s)`](super::Sample) that are associated
    /// with this [`File`] (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// assert_eq!(file.samples().len(), 1);
    /// assert_eq!(
    ///     file.samples().into_iter().next().unwrap().name(),
    ///     "SampleName001"
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn samples(&self) -> &NonEmpty<crate::sample::Identifier> {
        &self.samples
    }

    /// Gets the [gateway(s)](AnonymousOrReference) for the [`File`] (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// let gateways = file.gateways().unwrap();
    /// assert_eq!(gateways.len(), 1);
    /// let gateway = gateways.into_iter().next().unwrap();
    /// assert!(matches!(gateway, AnonymousOrReference::Anonymous { .. }));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gateways(&self) -> Option<&NonEmpty<gateway::AnonymousOrReference>> {
        self.gateways.as_ref()
    }

    /// Gets the [`Metadata`] for the [`File`] (if it exists, by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// assert!(file.metadata().is_some());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Generates a random [`File`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let organization = Organization::new(
    ///     "example-organization"
    ///         .parse::<organization::Identifier>()
    ///         .unwrap(),
    ///     "Example Organization"
    ///         .parse::<organization::Name>()
    ///         .unwrap(),
    ///     None,
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
    ///     None,
    /// );
    ///
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::random(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     sample_id,
    /// );
    ///
    /// assert_eq!(file.gateways().unwrap().len(), 1);
    /// ```
    pub fn random(identifier: Identifier, sample: crate::sample::Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            samples: NonEmpty::new(sample),
            gateways: match rng.gen_bool(0.9) {
                true => Some(NonEmpty::new(AnonymousOrReference::Anonymous {
                    gateway: crate::Gateway::Open {
                        link: Link::Direct {
                            url: "https://example.com".parse::<Url>().unwrap(),
                        },
                    },
                })),
                false => Some(NonEmpty::new(AnonymousOrReference::Reference {
                    gateway: String::from("gateway"),
                })),
            },
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random()),
                false => None,
            },
        }
    }
}

impl Entity for File {}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Samples are sorted purely by identifier: the values contained _within_ a
// [`Sample`] are not relevant to the sort order. They are, however, relevant
// to equality—thus, why [`Eq`] and [`PartialEq`] are derived.
impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use ccdi_cde as cde;

    use crate::namespace;
    use crate::organization;
    use crate::sample;
    use crate::Namespace;

    use super::*;

    #[test]
    fn it_orders_samples_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::new(
            namespace::Identifier::new(
                organization::Identifier::try_new("example-organization").unwrap(),
                namespace::identifier::Name::try_new("ExampleNamespace").unwrap(),
            ),
            "support@example.com",
            Some(
                "ExampleNamespace"
                    .parse::<namespace::Description>()
                    .unwrap(),
            ),
            None,
        );

        let a = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("A.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );
        let b = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("B.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("C.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );
        let b = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("B.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );
        let bar = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );

        assert_eq!(foo.cmp(&bar), Ordering::Equal);
    }

    #[test]
    fn it_tests_equality_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::new(
            namespace::Identifier::new(
                organization::Identifier::try_new("example-organization").unwrap(),
                namespace::identifier::Name::try_new("ExampleNamespace").unwrap(),
            ),
            "support@example.com",
            Some(
                "ExampleNamespace"
                    .parse::<namespace::Description>()
                    .unwrap(),
            ),
            None,
        );

        let foo = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );
        let bar = File::new(
            Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            None,
            None,
        );

        assert!(foo == bar);
    }
}
