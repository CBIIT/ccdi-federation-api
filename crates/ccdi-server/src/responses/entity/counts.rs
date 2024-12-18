//! Paged counts included within every entity endpoint.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Counts that summarize the contents of a paged entity response.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::entity::Counts)]
pub struct Counts {
    /// The number of entities within the currently selected page in the result set.
    current: usize,

    /// The number of entities across all pages in the result set.
    all: usize,
}

impl Counts {
    /// Creates a new [`Counts`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccdi_server as server;
    ///
    /// use server::responses::entity::Counts;
    ///
    /// let summary = Counts::new(1, 10);
    /// ```
    pub fn new(current: usize, all: usize) -> Self {
        Self { current, all }
    }
}
