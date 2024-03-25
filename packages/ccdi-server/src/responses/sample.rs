use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use crate::responses::entity::Counts;
use crate::responses::entity::Summary;

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
/// 1. The namespace organization field should be sorted alphabetically. If all
///    values for the namespace organization are equal, continue on to the next
///    sorting criteria.
/// 2. The namespace name field should be sorted alphabetically. If all values
///    for the namespace names are equal, continue on to the next sorting
///    criteria.
/// 3. The entity name should be sorted alphabetically.
///
/// Since the `namespace` and `name` identifiers should always uniquely apply to
/// a single entity, this should always resolve to an ordering.
///
/// If there is a provided sort order, use that instead.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Samples)]
pub struct Samples {
    /// A summary of this paged result set.
    #[schema(value_type = responses::entity::Summary)]
    summary: Summary,

    /// The samples.
    #[schema(nullable = false)]
    data: Vec<models::Sample>,
}

impl From<(Vec<models::Sample>, usize)> for Samples {
    fn from((samples, count): (Vec<models::Sample>, usize)) -> Self {
        let counts = Counts::new(samples.len(), count);

        Self {
            summary: Summary::new(counts),
            data: samples,
        }
    }
}
