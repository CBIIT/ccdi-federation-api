//! Responses related to files.

use std::collections::HashSet;
use std::ops::Sub;

use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod data;

pub use data::Data;

/// An error related to a [`Files`] reponse.
#[derive(Debug)]
pub enum Error {
    /// One or more references to a gateway were included in a file but not in
    /// the named gateways.
    MissingGateways(Vec<String>),

    /// One or more named gateways were included in the named gateways that were
    /// never referenced by a file.
    ExtraneousGateways(Vec<String>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingGateways(names) => write!(f, "missing gateways: {}", names.join(", ")),
            Error::ExtraneousGateways(names) => {
                write!(f, "extraneous gateways: {}", names.join(", "))
            }
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

/// A response representing multiple files known about by the server.
///
/// When no sort order is provided, files **must** be ordered by the primary
/// identifier. This means that, when comparing two identifiers:
///
/// 1. The `namespace` field should be sorted alphabetically. If there is a
///    clear order, return that. Else, if the namespaces are equal,
/// 2. The `name` field should be sorted alphabetically. Since the `namespace`
///    and the `name` should always uniquely apply to a single entity, this
///    should always resolve to an ordering.
///
/// Of course, if there is a provided sort order, use that instead.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Files)]
pub struct Files {
    /// The data.
    #[schema(value_type = responses::file::Data, nullable = false)]
    data: Data,
}

impl Files {
    /// Creates a new [`Files`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::gateway::Link;
    /// use models::gateway::Named;
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
    /// let sample_id = sample::Identifier::new(namespace.id().clone(), "SampleName001");
    ///
    /// let file = File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id),
    ///     NonEmpty::new(AnonymousOrReference::Reference {
    ///         gateway: String::from("name"),
    ///     }),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// let gateway = Named::new(
    ///     String::from("name"),
    ///     Gateway::Open {
    ///         link: Link::Direct {
    ///             url: "https://example.com".parse::<Url>().unwrap(),
    ///         },
    ///     },
    /// );
    ///
    /// let files = server::responses::file::data::Files::from((vec![file], 10usize));
    /// let response = server::responses::Files::try_new(files, vec![gateway]).unwrap();
    /// ```
    pub fn try_new(
        files: impl Into<data::Files>,
        gateways: impl Into<data::Gateways>,
    ) -> Result<Self> {
        let files: data::Files = files.into();
        let gateways: data::Gateways = gateways.into();

        let expected_gateways = files
            .expected_gateways()
            .into_iter()
            .collect::<HashSet<_>>();
        let found_gateways = gateways
            .iter()
            .map(|gateway| gateway.name().to_string())
            .collect::<HashSet<_>>();

        let missing_gateways = expected_gateways.sub(&found_gateways);
        if !missing_gateways.is_empty() {
            return Err(Error::MissingGateways(
                missing_gateways.into_iter().collect_vec(),
            ));
        }

        let extraneous_gateways = found_gateways.sub(&expected_gateways);
        if !extraneous_gateways.is_empty() {
            return Err(Error::ExtraneousGateways(
                extraneous_gateways.into_iter().collect_vec(),
            ));
        }

        Ok(Self {
            data: Data::new(files, gateways),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ccdi_models::file::Identifier;
    use ccdi_models::gateway::AnonymousOrReference;
    use ccdi_models::gateway::Link;
    use ccdi_models::gateway::Named;
    use ccdi_models::namespace;
    use ccdi_models::organization;
    use ccdi_models::sample;
    use ccdi_models::File;
    use ccdi_models::Gateway;
    use ccdi_models::Namespace;
    use ccdi_models::Url;
    use nonempty::NonEmpty;

    #[test]
    fn missing_gateways() {
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
        );

        let file = File::new(
            Identifier::new(
                namespace.id().clone(),
                ccdi_cde::v1::file::Name::new("Foo.txt"),
            ),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            NonEmpty::new(AnonymousOrReference::Reference {
                gateway: String::from("name"),
            }),
            None,
        );

        let gateway = Named::new(
            String::from("another-name"),
            Gateway::Open {
                link: Link::Direct {
                    url: "https://example.com".parse::<Url>().unwrap(),
                },
            },
        );

        let err =
            Files::try_new(data::Files::from((vec![file], 10usize)), vec![gateway]).unwrap_err();
        assert!(matches!(err, Error::MissingGateways(_)));
        assert_eq!(err.to_string(), String::from("missing gateways: name"));
    }

    #[test]
    fn extraneous_gateways() {
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
        );

        let file = File::new(
            Identifier::new(
                namespace.id().clone(),
                ccdi_cde::v1::file::Name::new("Foo.txt"),
            ),
            NonEmpty::new(sample::Identifier::new(
                namespace.id().clone(),
                "SampleName001",
            )),
            NonEmpty::new(AnonymousOrReference::Reference {
                gateway: String::from("name"),
            }),
            None,
        );

        let gateways = vec![
            Named::new(
                String::from("name"),
                Gateway::Open {
                    link: Link::Direct {
                        url: "https://example.com".parse::<Url>().unwrap(),
                    },
                },
            ),
            Named::new(
                String::from("another-name"),
                Gateway::Open {
                    link: Link::Direct {
                        url: "https://example.com".parse::<Url>().unwrap(),
                    },
                },
            ),
        ];

        let err = Files::try_new(data::Files::from((vec![file], 10usize)), gateways).unwrap_err();
        assert!(matches!(err, Error::ExtraneousGateways(_)));
        assert_eq!(
            err.to_string(),
            String::from("extraneous gateways: another-name")
        );
    }
}
