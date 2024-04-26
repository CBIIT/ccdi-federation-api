//! Responses for grouping by fields for files and counting them.

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
