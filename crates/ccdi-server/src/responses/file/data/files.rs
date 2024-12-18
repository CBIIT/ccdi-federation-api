use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models::gateway::AnonymousOrReference;

use crate::responses::entity::Counts;
use crate::responses::entity::Summary;
use crate::responses::file::File;

/// Files within a [`Data`](super::Data) response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::file::data::Files)]
pub struct Files {
    /// A summary of this paged result set.
    #[schema(value_type = responses::entity::Summary)]
    summary: Summary,

    /// The files.
    #[schema(nullable = false, value_type = Vec<responses::File>)]
    files: Vec<File>,
}

impl Files {
    /// Gets the names of the expected gateways from a [`Files`].
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
    /// use models::Gateway;
    /// use models::Namespace;
    /// use models::Organization;
    /// use models::Url;
    /// use nonempty::NonEmpty;
    /// use server::responses::file::data::Files;
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
    /// let files = Files::from((
    ///     vec![
    ///         File::from(models::File::new(
    ///             Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Foo.txt")),
    ///             NonEmpty::new(sample_id.clone()),
    ///             NonEmpty::new(AnonymousOrReference::Reference {
    ///                 gateway: String::from("name"),
    ///             }),
    ///             Some(Metadata::random()),
    ///         )),
    ///         File::from(models::File::new(
    ///             Identifier::new(namespace.id().clone(), cde::v1::file::Name::new("Bar.txt")),
    ///             NonEmpty::new(sample_id),
    ///             NonEmpty::new(AnonymousOrReference::Reference {
    ///                 gateway: String::from("name"),
    ///             }),
    ///             Some(Metadata::random()),
    ///         )),
    ///     ],
    ///     10usize,
    /// ));
    ///
    /// let mut expected_gateways = files.expected_gateways();
    /// assert_eq!(expected_gateways.len(), 1);
    /// assert_eq!(expected_gateways.pop().unwrap(), String::from("name"));
    /// ```
    pub fn expected_gateways(&self) -> Vec<String> {
        self.files
            .iter()
            .flat_map(|file| file.gateways())
            .filter_map(|gateway| match gateway {
                AnonymousOrReference::Reference { gateway } => Some(gateway.to_owned()),
                _ => None,
            })
            .unique()
            .collect::<Vec<_>>()
    }
}

impl std::ops::Deref for Files {
    type Target = Vec<File>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl From<(Vec<File>, usize)> for Files {
    fn from((files, count): (Vec<File>, usize)) -> Self {
        let counts = Counts::new(files.len(), count);

        Self {
            summary: Summary::new(counts),
            files,
        }
    }
}
