use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use models::gateway::AnonymousOrReference;

/// Files within a [`Data`](super::Data) response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::file::data::Files)]
pub struct Files {
    /// The files.
    #[schema(nullable = false)]
    files: Vec<models::File>,
}

impl Files {
    /// Gets the names of the expected gateways from a [`Files`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::file::Identifier;
    /// use models::gateway::AnonymousOrReference;
    /// use models::sample;
    /// use models::File;
    /// use models::Namespace;
    /// use nonempty::NonEmpty;
    /// use server::responses::file::data::Files;
    ///
    /// let namespace = Namespace::try_new(
    ///     "organization",
    ///     "Example Organization",
    ///     "support@example.com",
    ///     None,
    /// )
    /// .unwrap();
    ///
    /// let files = Files::from(vec![
    ///     File::new(
    ///         Identifier::new(&namespace, "Foo.txt"),
    ///         NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///         NonEmpty::new(AnonymousOrReference::Reference {
    ///             gateway: String::from("name"),
    ///         }),
    ///     ),
    ///     File::new(
    ///         Identifier::new(&namespace, "Bar.txt"),
    ///         NonEmpty::new(sample::Identifier::new(&namespace, "SampleName001")),
    ///         NonEmpty::new(AnonymousOrReference::Reference {
    ///             gateway: String::from("name"),
    ///         }),
    ///     ),
    /// ]);
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
    type Target = Vec<models::File>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl From<Vec<models::File>> for Files {
    fn from(files: Vec<models::File>) -> Self {
        Self { files }
    }
}
