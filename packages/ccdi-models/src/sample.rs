//! Representations of samples.

use rand::thread_rng;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod identifier;
pub mod metadata;

pub use identifier::Identifier;
pub use metadata::Metadata;

use crate::gateway;
use crate::gateway::AnonymousOrReference;
use crate::gateway::Link;
use crate::Entity;
use crate::Url;
use nonempty::NonEmpty;

/// A sample.
///
/// **Note:** the `subject` identifier **must** match a
/// [`Subject`](super::Subject) that both (a) is listed in the
/// [`Subject`](super::Subject) index endpoint and (b) is able to be shown with
/// the [`Subject`](super::Subject) show endpoint.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::Sample)]
pub struct Sample {
    /// The primary identifier used by the site for this [`Sample`].
    ///
    /// This namespace pointed to by this identifier must also **ALWAYS** be
    /// included in results provided by the `/namespace` endpoint (and the
    /// subsequent `/namespace/<name>` endpoint). Failure to include the
    /// namespace in the results at that endpoint signifies non-compliance with
    /// the API.
    #[schema(value_type = models::sample::Identifier)]
    id: Identifier,

    /// The identifier for the subject from which this sample was derived.
    ///
    /// This identifier **must** match a [`Subject`](super::Subject) that both
    /// (a) is listed in the [`Subject`](super::Subject) index endpoint and (b)
    /// is able to be shown with the [`Subject`](super::Subject) show endpoint.
    #[schema(value_type = models::subject::Identifier)]
    subject: crate::subject::Identifier,

    /// One or more [gateways](AnonymousOrReference) through which this sample
    /// can be accessed.
    ///
    /// Gateways can be either [anonymous](AnonymousOrReference::Anonymous)
    /// ([gateways](crate::Gateway) with no name) or a
    /// [refererence](AnonymousOrReference::Reference) to a [named
    /// gateway](gateway::Named) ([gateways](crate::Gateway) with a name).
    ///
    /// **Anonymous** gateways are intended to be embedded directly within a
    /// returned sample in the `/sample` response object. They have no name and
    /// are only referred to by the sample within which they are embedded.
    ///
    /// **Named** gateways, on the other hand, are included in the `gateways`
    /// key in the `/sample` response object and referred to by name within a
    /// returned sample in the `/sample` response object. They are intended to
    /// be used when more than one sample references the same gateway. This
    /// mechanism is available to ensure that the gateway object does not need
    /// to be duplicated multiple times in the response in these cases.
    ///
    /// This field can contain multiple gateways to support scenarios where a
    /// sample is available through more than one mechanism. We expect that only
    /// one gateway will be returned in most responses (if at all).
    #[schema(
        value_type = Vec<models::gateway::AnonymousOrReference>,
        required = false,
        nullable = false,
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<NonEmpty<gateway::AnonymousOrReference>>,

    /// Metadata associated with this [`Sample`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(
        value_type = Option<models::sample::Metadata>,
        nullable = true
    )]
    metadata: Option<Metadata>,
}

impl Sample {
    /// Creates a new [`Sample`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let sample = Sample::new(
    ///     sample_id,
    ///     subject_id,
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        id: Identifier,
        subject: crate::subject::Identifier,
        gateways: Option<NonEmpty<gateway::AnonymousOrReference>>,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            id,
            subject,
            gateways,
            metadata,
        }
    }

    /// Gets the name for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let sample = Sample::new(
    ///     sample_id,
    ///     subject_id,
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Builder::default().build()),
    /// );
    ///
    /// assert_eq!(
    ///     sample.id().namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(sample.id().namespace().name().as_str(), "ExampleNamespace");
    /// assert_eq!(sample.id().name(), &String::from("SampleName001"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the identifier for the [`Subject`](super::Subject) from which this
    /// [`Sample`] was derived (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let sample = Sample::new(
    ///     sample_id,
    ///     subject_id,
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(Builder::default().build()),
    /// );
    /// assert_eq!(
    ///     sample.subject().namespace().organization().as_str(),
    ///     "example-organization"
    /// );
    /// assert_eq!(
    ///     sample.subject().namespace().name().as_str(),
    ///     "ExampleNamespace"
    /// );
    /// assert_eq!(sample.subject().name().as_str(), "SubjectName001");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn subject(&self) -> &crate::subject::Identifier {
        &self.subject
    }

    /// Gets the metadata for this [`Sample`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    ///
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample::metadata::Builder;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    /// let metadata = Builder::default().build();
    ///
    /// let sample = Sample::new(
    ///     sample_id,
    ///     subject_id,
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(metadata.clone()),
    /// );
    /// assert_eq!(sample.metadata(), Some(&metadata));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Gets the [gateway(s)](AnonymousOrReference) for the [`Sample`] (by reference).
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
    /// use models::sample::metadata::Builder;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    /// let metadata = Builder::default().build();
    ///
    /// let sample = Sample::new(
    ///     sample_id,
    ///     subject_id,
    ///     Some(NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: "https://example.com".parse::<Url>().unwrap(),
    ///             },
    ///         },
    ///     })),
    ///     Some(metadata.clone()),
    /// );
    ///
    /// let gateways = sample.gateways().unwrap();
    /// assert_eq!(gateways.len(), 1);
    /// let gateway = gateways.into_iter().next().unwrap();
    /// assert!(matches!(gateway, AnonymousOrReference::Anonymous { .. }));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gateways(&self) -> Option<&NonEmpty<gateway::AnonymousOrReference>> {
        self.gateways.as_ref()
    }

    /// Generates a random [`Sample`].
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
    /// use models::Sample;
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
    /// let subject_id = models::subject::Identifier::new(namespace.id().clone(), "SubjectName001");
    /// let sample_id = models::sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let sample = Sample::random(sample_id, subject_id);
    /// ```
    pub fn random(identifier: Identifier, subject: crate::subject::Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            subject,
            metadata: match rng.gen_bool(0.7) {
                true => Some(Metadata::random(identifier)),
                false => None,
            },
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
        }
    }
}

impl Entity for Sample {}

impl PartialOrd for Sample {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Samples are sorted purely by identifier: the values contained _within_ a
// [`Sample`] are not relevant to the sort order. They are, however, relevant
// to equality—thus, why [`Eq`] and [`PartialEq`] are derived.
impl Ord for Sample {
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

        let a = Sample::new(
            Identifier::new(namespace.id().clone(), "A"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectA"),
            ),
            None,
            None,
        );
        let b = Sample::new(
            Identifier::new(namespace.id().clone(), "B"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectB"),
            ),
            None,
            None,
        );

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = Sample::new(
            Identifier::new(namespace.id().clone(), "C"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectC"),
            ),
            None,
            None,
        );
        let b = Sample::new(
            Identifier::new(namespace.id().clone(), "B"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectB"),
            ),
            None,
            None,
        );

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = Sample::new(
            Identifier::new(namespace.id().clone(), "Name"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("Subject"),
            ),
            None,
            None,
        );
        let bar = Sample::new(
            Identifier::new(namespace.id().clone(), "Name"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("Subject"),
            ),
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

        let foo = Sample::new(
            Identifier::new(namespace.id().clone(), "B"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectB"),
            ),
            None,
            None,
        );
        let bar = Sample::new(
            Identifier::new(namespace.id().clone(), "B"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectB"),
            ),
            None,
            None,
        );

        assert!(foo == bar);

        let foo = Sample::new(
            Identifier::new(namespace.id().clone(), "A"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectA"),
            ),
            None,
            None,
        );
        let bar = Sample::new(
            Identifier::new(namespace.id().clone(), "B"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectB"),
            ),
            None,
            None,
        );

        assert!(foo != bar);

        let foo = Sample::new(
            Identifier::new(namespace.id().clone(), "Name"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectName"),
            ),
            None,
            Some(metadata::Builder::default().build()),
        );
        let bar = Sample::new(
            Identifier::new(namespace.id().clone(), "Name"),
            crate::subject::Identifier::new(
                namespace.id().clone(),
                cde::v1::subject::Name::new("SubjectName"),
            ),
            None,
            None,
        );

        assert!(foo != bar);
    }
}
