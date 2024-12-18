//! Responses related to a summary of entities.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// Counts included in a summary endpoint.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::summary::Counts)]
pub struct Counts {
    total: usize,
}

/// A summary response for an entity endpoint.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Summary)]
pub struct Summary {
    #[schema(value_type = responses::summary::Counts)]
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
    /// use server::responses::Summary;
    ///
    /// let summary = Summary::new(1);
    /// ```
    pub fn new(total: usize) -> Self {
        Self {
            counts: Counts { total },
        }
    }
}
