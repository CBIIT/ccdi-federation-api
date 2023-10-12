//! Counts of totals.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Total count of some entity as reported alongside an API call.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = models::count::Total)]
pub struct Total {
    /// The total number of entities returned in the API call.
    total: usize,
}

impl From<usize> for Total {
    fn from(value: usize) -> Self {
        Self { total: value }
    }
}
