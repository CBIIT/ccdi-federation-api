use itertools::Itertools;
use models::gateway;
use models::gateway::Link;
use models::Gateway;
use models::Url;
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
/// 1. The namespace organization field should be sorted alphabetically. If all
///    values for the namespace organization are equal, continue on to the next
///    sorting criteria.
/// 2. The namespace name field should be sorted alphabetically. If all
///    values for the namespace names are equal, continue on to the next
///    sorting criteria.
/// 3. The entity name should be sorted alphabetically.
///
/// Since the `namespace` and `name` identifiers should always uniquely apply to
/// a single entity, this should always resolve to an ordering.
///
/// If there is a provided sort order, use that instead.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(as = responses::Subjects)]
pub struct Subjects {
    /// A summary of this paged result set.
    #[schema(value_type = responses::entity::Summary)]
    summary: Summary,

    /// The subjects.
    #[schema(nullable = false)]
    data: Vec<models::Subject>,

    // The gateways.
    #[schema(nullable = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<Vec<models::gateway::Named>>,
}

impl From<(Vec<models::Subject>, usize)> for Subjects {
    fn from((subjects, total): (Vec<models::Subject>, usize)) -> Self {
        let gateways = subjects
            .iter()
            .flat_map(|subject| subject.gateways())
            .flatten()
            .flat_map(|gateway| gateway.as_reference().map(|gateway| gateway.to_owned()))
            .unique()
            .map(|name| {
                gateway::Named::new(
                    name,
                    Gateway::Open {
                        link: Link::Direct {
                            url: "https://example.com".parse::<Url>().unwrap(),
                        },
                    },
                )
            })
            .collect::<Vec<_>>();

        let counts = Counts::new(subjects.len(), total);

        Self {
            summary: Summary::new(counts),
            data: subjects,
            gateways: match gateways.is_empty() {
                true => None,
                false => Some(gateways),
            },
        }
    }
}
