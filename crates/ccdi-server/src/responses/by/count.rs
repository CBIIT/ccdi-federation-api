//! Responses for grouping by fields and counting them.

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

pub mod file;
pub mod sample;
pub mod subject;

/// A value along with the number of counted entities for that value.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::by::count::ValueCount)]
pub struct ValueCount {
    /// The value.
    pub value: Value,

    /// The number of times the value was counted.
    pub count: usize,
}
