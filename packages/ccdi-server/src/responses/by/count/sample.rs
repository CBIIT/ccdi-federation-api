//! Responses for grouping by fields for samples and counting them.

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

/// A response for grouping [`Sample`](models::Sample)s by a metadata field
/// and then summing the counts.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::Samples)]
pub struct Samples {
    #[serde(flatten)]
    total: models::count::Total,

    values: IndexMap<String, usize>,
}

impl From<IndexMap<String, usize>> for Samples {
    /// Creates a new [`Samples`] from an [`IndexMap<String, usize>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexMap;
    ///
    /// use ccdi_models as models;
    /// use ccdi_server as server;
    ///
    /// use models::count::Total;
    /// use server::responses::by::count::Samples;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("Diagnosis".into(), 10);
    /// map.insert("Relapse".into(), 10);
    /// map.insert("Metastasis".into(), 10);
    ///
    /// let samples = Samples::from(map);
    /// ```
    fn from(values: IndexMap<String, usize>) -> Self {
        let total = values.values().sum::<usize>();
        Self {
            total: models::count::Total::from(total),
            values,
        }
    }
}