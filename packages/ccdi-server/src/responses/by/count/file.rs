//! Responses for grouping by fields for files and counting them.

use ccdi_models::namespace;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::responses::by::count::ValueCount;

/// A set of results from grouping [`Files`](ccdi_models::File) by a specified
/// metadata field and then summing the counts for each value (along with computing a
/// total count).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::file::Results)]
pub struct Results {
    /// The total number of counts in this result set.
    pub total: usize,

    /// The total number of entries that are missing values. In this context,
    /// "missing" means either (a) the individual metadata key is missing or (b)
    /// the entire metadata object is missing.
    pub missing: usize,

    /// The counts per value observed for the result set.
    #[schema(value_type = Vec<responses::by::count::ValueCount>)]
    pub values: Vec<ValueCount>,
}

impl Results {
    /// Creates a new [`Results`] from an [`Vec<ValueCount>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use server::responses::by::count::file::Results;
    /// use server::responses::by::count::ValueCount;
    ///
    /// let mut counts = vec![
    ///     ValueCount {
    ///         value: "BAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "CRAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "VCF".into(),
    ///         count: 10,
    ///     },
    /// ];
    ///
    /// let results = Results::new(counts, 10);
    ///
    /// assert_eq!(results.total, 40);
    /// ```
    pub fn new(values: Vec<ValueCount>, missing: usize) -> Self {
        let total = values.iter().map(|result| result.count).sum::<usize>() + missing;

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
#[schema(as = responses::by::count::file::NamespacePartitionedResult)]
pub struct NamespacePartitionedResult {
    #[schema(value_type = models::namespace::Identifier)]
    namespace: namespace::Identifier,

    #[schema(value_type = responses::by::count::file::Results)]
    results: Results,
}

impl NamespacePartitionedResult {
    /// Creates a new [`NamespacePartitionedResult`].
    ///
    /// # Example
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    /// use server::responses::by::count::file::NamespacePartitionedResult;
    /// use server::responses::by::count::file::Results;
    /// use server::responses::by::count::ValueCount;
    ///
    /// let mut counts = vec![
    ///     ValueCount {
    ///         value: "BAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "CRAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "VCF".into(),
    ///         count: 10,
    ///     },
    /// ];
    ///
    /// let results = Results::new(counts, 10);
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
    as = responses::by::count::file::NamespacePartitionedResults,
    value_type = Vec<responses::by::count::file::NamespacePartitionedResult>
)]
pub struct NamespacePartitionedResults(Vec<NamespacePartitionedResult>);

impl From<Vec<NamespacePartitionedResult>> for NamespacePartitionedResults {
    /// Creates a new [`NamespacePartitionedResults`].
    ///
    /// # Example
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::namespace;
    /// use models::namespace::Identifier;
    /// use models::organization;
    /// use models::Organization;
    /// use server::responses::by::count::file::NamespacePartitionedResult;
    /// use server::responses::by::count::file::NamespacePartitionedResults;
    /// use server::responses::by::count::file::Results;
    /// use server::responses::by::count::ValueCount;
    ///
    /// let mut counts = vec![
    ///     ValueCount {
    ///         value: "BAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "CRAM".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "VCF".into(),
    ///         count: 10,
    ///     },
    /// ];
    ///
    /// let results = Results::new(counts, 10);
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
/// [`Files`](ccdi_models::File).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::file::Response)]
#[serde(untagged)]
pub enum Response {
    /// A set of results that have _not_ been partitioned.
    #[schema(value_type = responses::by::count::file::Results)]
    Unpartitioned(Results),

    /// A set of results that have been partitioned by
    /// [`Namespace`](ccdi_models::Namespace).
    #[schema(value_type = responses::by::count::file::NamespacePartitionedResults)]
    NamespacePartitioned(NamespacePartitionedResults),
}
