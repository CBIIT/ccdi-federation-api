//! Responses for grouping by fields for subjects and counting them.

use ccdi_models::namespace;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A response for grouping [`Subject`](ccdi_models::Subject)s by a metadata field
/// and then summing the counts.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::subject::Results)]
pub struct Results {
    /// The total number of counts in this result set.
    pub total: usize,

    /// The counts per value observed for the result set.
    pub values: IndexMap<String, usize>,
}

impl From<IndexMap<String, usize>> for Results {
    /// Creates a new [`Results`] from an [`IndexMap<String, usize>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexMap;
    ///
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use server::responses::by::count::subject::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("U".into(), 18);
    /// map.insert("F".into(), 37);
    /// map.insert("M".into(), 26);
    /// map.insert("UNDIFFERENTIATED".into(), 31);
    ///
    /// let results = Results::from(map);
    /// ```
    fn from(values: IndexMap<String, usize>) -> Self {
        let total = values.values().sum::<usize>();
        Self { total, values }
    }
}

/// A result that associates a namespace with the [`Results`] belonging to that
/// [`Namespace`](ccdi_models::Namespace).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::subject::NamespacePartitionedResult)]
pub struct NamespacePartitionedResult {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

    #[schema(value_type = responses::by::count::subject::Results)]
    results: Results,
}

impl NamespacePartitionedResult {
    /// Creates a new [`NamespacePartitionedResult`].
    ///
    /// # Example
    ///
    /// ```
    /// use indexmap::IndexMap;
    ///
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    /// use server::responses::by::count::subject::NamespacePartitionedResult;
    /// use server::responses::by::count::subject::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("U".into(), 18);
    /// map.insert("F".into(), 37);
    /// map.insert("M".into(), 26);
    /// map.insert("UNDIFFERENTIATED".into(), 31);
    ///
    /// let results = Results::from(map);
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
    /// let identifier = Identifier::new(
    ///     organization.id().clone(),
    ///     "ExampleNamespace"
    ///         .parse::<namespace::identifier::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// let partitioned_results = NamespacePartitionedResult::new(identifier, results);
    /// ```
    pub fn new(namespace: namespace::Identifier, results: Results) -> Self {
        Self { namespace, results }
    }
}

/// A set of [`NamespacePartitionedResults`].
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    as = responses::by::count::subject::NamespacePartitionedResults,
    value_type = Vec<responses::by::count::subject::NamespacePartitionedResult>
)]
pub struct NamespacePartitionedResults(Vec<NamespacePartitionedResult>);

impl From<Vec<NamespacePartitionedResult>> for NamespacePartitionedResults {
    /// Creates a new [`NamespacePartitionedResults`].
    ///
    /// # Example
    ///
    /// ```
    /// use indexmap::IndexMap;
    ///
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    /// use server::responses::by::count::subject::NamespacePartitionedResult;
    /// use server::responses::by::count::subject::NamespacePartitionedResults;
    /// use server::responses::by::count::subject::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("U".into(), 18);
    /// map.insert("F".into(), 37);
    /// map.insert("M".into(), 26);
    /// map.insert("UNDIFFERENTIATED".into(), 31);
    ///
    /// let results = Results::from(map);
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
    /// let identifier = Identifier::new(
    ///     organization.id().clone(),
    ///     "ExampleNamespace"
    ///         .parse::<namespace::identifier::Name>()
    ///         .unwrap(),
    /// );
    ///
    /// let partitioned_results = NamespacePartitionedResult::new(identifier, results);
    /// let results = NamespacePartitionedResults::from(vec![partitioned_results]);
    /// ```
    fn from(value: Vec<NamespacePartitionedResult>) -> Self {
        Self(value)
    }
}

/// A unified response for counting by a particular field for
/// [`Subjects`](ccdi_models::Subject).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::subject::Response)]
#[serde(untagged)]
pub enum Response {
    /// A set of results that have _not_ been partitioned.
    #[schema(value_type = responses::by::count::subject::Results)]
    Unpartitioned(Results),

    /// A set of results that have been partitioned by
    /// [`Namespace`](ccdi_models::Namespace).
    #[schema(value_type = responses::by::count::subject::NamespacePartitionedResults)]
    NamespacePartitioned(NamespacePartitionedResults),
}
