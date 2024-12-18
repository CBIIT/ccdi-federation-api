//! Cross-entity elements.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod counts;
pub use counts::Counts;

/// A summary of a paged entity response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::entity::Summary)]
pub struct Summary {
    #[schema(value_type = responses::entity::Counts)]
    counts: Counts,
}

impl Summary {
    /// Creates a new [`Summary`] response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::entity::Counts;
    /// use server::responses::entity::Summary;
    ///
    /// let counts = Counts::new(1, 10);
    /// let summary = Summary::new(counts);
    /// ```
    pub fn new(counts: Counts) -> Self {
        Self { counts }
    }
}
