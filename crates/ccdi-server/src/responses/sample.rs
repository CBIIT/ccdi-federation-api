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

    // The gateways.
    #[schema(nullable = false)]
    #[serde(skip_serializing_if = "Option::is_none")]
    gateways: Option<Vec<models::gateway::Named>>,
}

impl From<(Vec<models::Sample>, usize)> for Samples {
    fn from((samples, total): (Vec<models::Sample>, usize)) -> Self {
        let gateways = samples
            .iter()
            .flat_map(|sample| sample.gateways())
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

        let counts = Counts::new(samples.len(), total);

        Self {
            summary: Summary::new(counts),
            data: samples,
            gateways: match gateways.is_empty() {
                true => None,
                false => Some(gateways),
            },
        }
    }
}
