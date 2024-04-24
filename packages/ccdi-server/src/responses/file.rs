//! Responses related to files.

use itertools::Itertools as _;
use models::gateway;
use models::gateway::Link;
use models::Gateway;
use models::Url;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use crate::responses::entity::Counts;
use crate::responses::entity::Summary;

/// A response representing a single [`File`](models::File).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::File)]
pub struct File {
    /// File.
    #[serde(flatten)]
    inner: models::File,
}

impl File {
    /// Consumes `self` to return the inner [`File`](models::File).
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_cde as cde;
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use nonempty::NonEmpty;
    ///
    /// use models::file;
    /// use models::file::Identifier;
    /// use models::file::Metadata;
    /// use models::gateway::AnonymousOrReference;
    /// use models::namespace;
    /// use models::organization;
    /// use models::sample;
    /// use models::Namespace;
    /// use models::Organization;
    /// use server::responses::File;
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
    /// let raw_file = models::File::new(
    ///     Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///     NonEmpty::new(sample_id.clone()),
    ///     Some(NonEmpty::new(AnonymousOrReference::Reference {
    ///         gateway: String::from("name"),
    ///     })),
    ///     Some(Metadata::random()),
    /// );
    ///
    /// let file = File::from(raw_file.clone());
    /// assert_eq!(file.into_inner(), raw_file);
    /// ```
    pub fn into_inner(self) -> models::File {
        self.inner
    }
}

impl std::ops::Deref for File {
    type Target = models::File;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<models::File> for File {
    fn from(value: models::File) -> Self {
        Self { inner: value }
    }
}

/// A response representing multiple files known about by the server.
///
/// When no sort order is provided, files **must** be ordered by the primary
/// identifier. This means that, when comparing two identifiers:
///
/// 1. The namespace organization field should be sorted alphabetically. If all
///    values for the namespace organization are equal, continue on to the next
///    sorting criteria.
/// 2. The namespace name field should be sorted alphabetically. If all
///    values for the namespace names are equal, continue on to the next
///    sorting criteria.
/// 3. The entity name should be sorted alphabetically.
///
/// Since the `namespace` and `name` identifiers should always uniquely apply to
/// a single entity, this should always resolve to an ordering.
///
/// If there is a provided sort order, use that instead.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Files)]
pub struct Files {
    /// A summary of this paged result set.
    #[schema(value_type = responses::entity::Summary)]
    summary: Summary,

    /// The files.
    #[schema(nullable = false, value_type = Vec<responses::File>)]
    data: Vec<models::File>,

    // The gateways.
    #[schema(nullable = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<Vec<models::gateway::Named>>,
}

impl From<(Vec<models::File>, usize)> for Files {
    fn from((files, total): (Vec<models::File>, usize)) -> Self {
        let gateways = files
            .iter()
            .flat_map(|file| file.gateways())
            .flatten()
            .flat_map(|gateway| gateway.as_reference().map(|gateway| gateway.to_owned()))
            .unique()
            .map(|name| {
                gateway::Named::new(
                    name,
                    Gateway::Open {
                        link: Link::Direct {
                            url: "https://example.com".parse::<Url>().unwrap(),
                        },
                    },
                )
            })
            .collect::<Vec<_>>();

        let counts = Counts::new(files.len(), total);

        Self {
            summary: Summary::new(counts),
            data: files,
            gateways: match gateways.is_empty() {
                true => None,
                false => Some(gateways),
            },
        }
    }
}
