use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use models::count::Total;

/// A response representing a single [`Sample`](models::Sample).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Sample)]
pub struct Sample {
    /// Sample.
    #[serde(flatten)]
    inner: models::Sample,
}

/// A response representing multiple samples known about by the server with a
/// summarized total count.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Samples)]
pub struct Samples {
    /// The total number of samples in this response.
    #[schema(value_type = models::count::Total)]
    count: Total,

    /// The samples, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    samples: Option<Vec<models::Sample>>,
}

impl From<Vec<models::Sample>> for Samples {
    fn from(samples: Vec<models::Sample>) -> Self {
        Self {
            count: samples.len().into(),
            samples: Some(samples),
        }
    }
}
