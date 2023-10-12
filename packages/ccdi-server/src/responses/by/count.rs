//! Responses for grouping by fields and counting them.

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use models::count::Total;

/// A response for grouping [`Subject`](models::Subject)s by a metadata field
/// and then summing the counts.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Subjects {
    #[serde(flatten)]
    total: Total,

    values: IndexMap<String, usize>,
}

impl From<IndexMap<String, usize>> for Subjects {
    /// Creates a new [`Subjects`] from an [`IndexMap<String, usize>`].
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
    /// use server::responses::by::count::Subjects;
    ///
    /// let mut map = IndexMap::<String, usize>::new();
    /// map.insert("U".into(), 18);
    /// map.insert("F".into(), 37);
    /// map.insert("M".into(), 26);
    /// map.insert("UNDIFFERENTIATED".into(), 31);
    ///
    /// let subjects = Subjects::from(map);
    /// ```
    fn from(values: IndexMap<String, usize>) -> Self {
        let total = values.values().sum::<usize>();
        Self {
            total: Total::from(total),
            values,
        }
    }
}
