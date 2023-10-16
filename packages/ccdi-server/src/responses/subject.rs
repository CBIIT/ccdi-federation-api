use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use ccdi_models as models;

use models::count::Total;

/// A response representing a single [`Subject`](models::Subject).
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Subject)]
pub struct Subject {
    /// Subject.
    #[serde(flatten)]
    inner: models::Subject,
}

/// A response representing multiple subjects known about by the server with a
/// summarized total count.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Subjects)]
pub struct Subjects {
    /// The total number of subjects in this response.
    #[schema(value_type = models::count::Total)]
    count: Total,

    /// The subjects, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    subjects: Option<Vec<models::Subject>>,
}

impl From<Vec<models::Subject>> for Subjects {
    fn from(subjects: Vec<models::Subject>) -> Self {
        Self {
            count: subjects.len().into(),
            subjects: Some(subjects),
        }
    }
}
