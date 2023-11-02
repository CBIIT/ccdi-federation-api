use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

/// A response representing a single [`Sample`](models::Sample).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Sample)]
pub struct Sample {
    /// Sample.
    #[serde(flatten)]
    inner: models::Sample,
}

/// A response representing multiple samples known about by the server.
///
/// When no sort order is provided, samples **must** be ordered by the primary
/// identifier. This means that, when comparing two identifiers:
///
/// 1. The `namespace` field should be sorted alphabetically. If there is a
///    clear order, return that. Else, if the namespaces are equal,
/// 2. The `name` field should be sorted alphabetically. Since the `namespace`
///    and the `name` should always uniquely apply to a single entity, this
///    should always resolve to an ordering.
///
/// Of course, if there is a provided sort order, use that instead.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Samples)]
pub struct Samples {
    /// The samples.
    #[schema(nullable = false)]
    data: Vec<models::Sample>,
}

impl From<Vec<models::Sample>> for Samples {
    fn from(samples: Vec<models::Sample>) -> Self {
        Self { data: samples }
    }
}
