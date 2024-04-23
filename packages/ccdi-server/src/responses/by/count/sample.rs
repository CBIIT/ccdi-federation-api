//! Responses for grouping by fields for samples and counting them.

use ccdi_models::namespace;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// A set of results from grouping [`Samples`](ccdi_models::Sample) by a specified
/// metadata field and then summing the counts for each value (along with computing a
/// total count).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::sample::Results)]
pub struct Results {
    /// The total number of counts in this result set.
    pub total: usize,

    /// The total number of entries that are missing values. In this context,
    /// "missing" means either (a) the individual metadata key is missing or (b)
    /// the entire metadata object is missing.
    pub missing: usize,

    /// The counts per value observed for the result set.
    pub values: IndexMap<String, usize>,
}

impl Results {
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
    /// use server::responses::by::count::sample::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("Diagnosis".into(), 10);
    /// map.insert("Relapse".into(), 10);
    /// map.insert("Metastasis".into(), 10);
    ///
    /// let results = Results::new(map, 10);
    /// ```
    pub fn new(values: IndexMap<String, usize>, missing: usize) -> Self {
        let total = values.values().sum::<usize>() + missing;

        Self {
            total,
            missing,
            values,
        }
    }
}

/// A result that associates a namespace with the [`Results`] belonging to that
/// [`Namespace`](ccdi_models::Namespace).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::sample::NamespacePartitionedResult)]
pub struct NamespacePartitionedResult {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

    #[schema(value_type = responses::by::count::sample::Results)]
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
    /// use server::responses::by::count::sample::NamespacePartitionedResult;
    /// use server::responses::by::count::sample::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("Diagnosis".into(), 10);
    /// map.insert("Relapse".into(), 10);
    /// map.insert("Metastasis".into(), 10);
    ///
    /// let results = Results::new(map, 10);
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
    as = responses::by::count::sample::NamespacePartitionedResults,
    value_type = Vec<responses::by::count::sample::NamespacePartitionedResult>
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
    /// use server::responses::by::count::sample::NamespacePartitionedResult;
    /// use server::responses::by::count::sample::NamespacePartitionedResults;
    /// use server::responses::by::count::sample::Results;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("Diagnosis".into(), 10);
    /// map.insert("Relapse".into(), 10);
    /// map.insert("Metastasis".into(), 10);
    ///
    /// let results = Results::new(map, 10);
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
/// [`Samples`](ccdi_models::Sample).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::sample::Response)]
#[serde(untagged)]
pub enum Response {
    /// A set of results that have _not_ been partitioned.
    #[schema(value_type = responses::by::count::sample::Results)]
    Unpartitioned(Results),

    /// A set of results that have been partitioned by
    /// [`Namespace`](ccdi_models::Namespace).
    #[schema(value_type = responses::by::count::sample::NamespacePartitionedResults)]
    NamespacePartitioned(NamespacePartitionedResults),
}
