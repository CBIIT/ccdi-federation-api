//! Closed gateways.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod status;

pub use status::Status;

/// A closed gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::gateway::Closed)]
pub struct Closed {
    /// A Markdown field (formatted according to the [CommonMark] standard) that
    /// describes the gateway.
    ///
    /// At a minimum, a description of what the gateway is and why it is closed
    /// is recommended.
    ///
    /// **Note:** this field is required for a closed gateway. This was an
    /// intentional decision: gateways are intended to point users to external
    /// resources. When that is not possible, a gateway's only purpose can be to
    /// provide further information about the data source. Put another way, if
    /// one is attempting to construct a closed gateway with no description, it
    /// should be considered why a gateway is needed at all.
    ///
    /// [CommonMark]: https://commonmark.org
    description: String,

    /// The status of the gateway.
    #[serde(flatten)]
    #[schema(value_type = models::gateway::closed::Status)]
    status: Status,
}
