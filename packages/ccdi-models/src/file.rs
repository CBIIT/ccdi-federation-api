//! Representations of files.

use nonempty::NonEmpty;
use rand::thread_rng;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod identifier;

pub use identifier::Identifier;

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
    /// duplicated mulitple times in the response in these cases.
    ///
    /// This field can contain multiple gateways to support scenarios where a
    /// file is available through more than one mechanism. We expect that only
    /// one gateway will be returned in most responses.
    ///
    /// **Note:** a file must have at least one gateway. If the file has no
    /// gateway, then it should not be returned as part of this API.
    #[schema(value_type = Vec<models::gateway::AnonymousOrReference>)]
    gateways: NonEmpty<gateway::AnonymousOrReference>,
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
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let file = File::new(
    ///     Identifier::new(&namespace, "Foo.txt"),
    ///     NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///     NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: Url::try_from("https://example.com").unwrap(),
    ///             },
    ///         },
    ///     }),
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        id: Identifier,
        samples: NonEmpty<crate::sample::Identifier>,
        gateways: NonEmpty<gateway::AnonymousOrReference>,
    ) -> Self {
        Self {
            id,
            samples,
            gateways,
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
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let file = File::new(
    ///     Identifier::new(&namespace, "Foo.txt"),
    ///     NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///     NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: Url::try_from("https://example.com").unwrap(),
    ///             },
    ///         },
    ///     }),
    /// );
    ///
    /// assert_eq!(file.id().namespace(), "organization");
    /// assert_eq!(file.id().name(), "Foo.txt");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn id(&self) -> &Identifier {
        &self.id
    }

    /// Gets the identifier(s) for the [`Sample(s)`](super::Subject) that are
    /// associated with this [`File`] (by reference).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    ///
    /// use models::file::Identifier;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let file = File::new(
    ///     Identifier::new(&namespace, "Foo.txt"),
    ///     NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///     NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: Url::try_from("https://example.com").unwrap(),
    ///             },
    ///         },
    ///     }),
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
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let file = File::new(
    ///     Identifier::new(&namespace, "Foo.txt"),
    ///     NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///     NonEmpty::new(AnonymousOrReference::Anonymous {
    ///         gateway: Gateway::Open {
    ///             link: Link::Direct {
    ///                 url: Url::try_from("https://example.com").unwrap(),
    ///             },
    ///         },
    ///     }),
    /// );
    ///
    /// assert_eq!(file.gateways().len(), 1);
    /// let gateway = file.gateways().into_iter().next().unwrap();
    /// assert!(matches!(gateway, AnonymousOrReference::Anonymous { .. }));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn gateways(&self) -> &NonEmpty<gateway::AnonymousOrReference> {
        &self.gateways
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
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::sample;
    /// use models::File;
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let file = File::random(
    ///     Identifier::new(&namespace, "Foo.txt"),
    ///     sample::Identifier::new(&namespace, "SampleName001"),
    /// );
    ///
    /// assert_eq!(file.gateways().len(), 1);
    /// ```
    pub fn random(identifier: Identifier, sample: crate::sample::Identifier) -> Self {
        let mut rng = thread_rng();

        Self {
            id: identifier.clone(),
            samples: NonEmpty::new(sample),
            gateways: match rng.gen_bool(0.9) {
                true => NonEmpty::new(AnonymousOrReference::Anonymous {
                    gateway: crate::Gateway::Open {
                        link: Link::Direct {
                            url: Url::try_from("https://example.com").unwrap(),
                        },
                    },
                }),
                false => NonEmpty::new(AnonymousOrReference::Reference {
                    gateway: String::from("gateway"),
                }),
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

    use crate::gateway::AnonymousOrReference;
    use crate::gateway::Link;
    use crate::sample;
    use crate::Gateway;
    use crate::Namespace;
    use crate::Url;

    use super::*;

    #[test]
    fn it_orders_samples_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::try_new(
            "organization",
            "Example Organization",
            "support@example.com",
            None,
        )
        .unwrap();

        let a = File::new(
            Identifier::new(&namespace, "A.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );
        let b = File::new(
            Identifier::new(&namespace, "B.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );

        assert_eq!(a.cmp(&b), Ordering::Less);

        let c = File::new(
            Identifier::new(&namespace, "C.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );
        let b = File::new(
            Identifier::new(&namespace, "B.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );

        assert_eq!(c.cmp(&b), Ordering::Greater);

        let foo = File::new(
            Identifier::new(&namespace, "Foo.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );
        let bar = File::new(
            Identifier::new(&namespace, "Foo.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );

        assert_eq!(foo.cmp(&bar), Ordering::Equal);
    }

    #[test]
    fn it_tests_equality_correctly() {
        // SAFETY: this is manually crafted to unwrap every time, as the
        // organization name conforms to the correct pattern.
        let namespace = Namespace::try_new(
            "organization",
            "Example Organization",
            "support@example.com",
            None,
        )
        .unwrap();

        let foo = File::new(
            Identifier::new(&namespace, "Foo.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );
        let bar = File::new(
            Identifier::new(&namespace, "Foo.txt"),
            NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
            NonEmpty::new(AnonymousOrReference::Anonymous {
                gateway: Gateway::Open {
                    link: Link::Direct {
                        url: Url::try_from("https://example.com").unwrap(),
                    },
                },
            }),
        );

        assert!(foo == bar);
    }
}