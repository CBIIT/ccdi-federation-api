//! Responses for grouping by fields for samples and counting them.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::responses::by::count::ValueCount;

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
    #[schema(value_type = Vec<responses::by::count::ValueCount>)]
    pub values: Vec<ValueCount>,
}

impl Results {
    /// Creates a new [`Results`] from a [`Vec<ValueCount>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use server::responses::by::count::sample::Results;
    /// use server::responses::by::count::ValueCount;
    ///
    /// let mut counts = vec![
    ///     ValueCount {
    ///         value: "Diagnosis".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "Relapse".into(),
    ///         count: 10,
    ///     },
    ///     ValueCount {
    ///         value: "Metastasis".into(),
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
