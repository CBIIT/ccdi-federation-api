use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use crate::responses::entity::Counts;
use crate::responses::entity::Summary;

/// A response representing a single [`Subject`](models::Subject).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Subject)]
pub struct Subject {
    /// Subject.
    #[serde(flatten)]
    inner: models::Subject,
}

/// A response representing multiple subjects known about by the server.
///
/// When no sort order is provided, subjects **must** be ordered by the primary
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
#[schema(as = responses::Subjects)]
pub struct Subjects {
    /// A summary of this paged result set.
    #[schema(value_type = responses::entity::Summary)]
    summary: Summary,

    /// The subjects.
    #[schema(nullable = false)]
    data: Vec<models::Subject>,
}

impl From<(Vec<models::Subject>, usize)> for Subjects {
    fn from((subjects, count): (Vec<models::Subject>, usize)) -> Self {
        let counts = Counts::new(subjects.len(), count);

        Self {
            summary: Summary::new(counts),
            data: subjects,
        }
    }
}
