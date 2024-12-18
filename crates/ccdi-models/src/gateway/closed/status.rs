use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// The status of a closed gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "status")]
#[schema(as = models::gateway::closed::Status)]
pub enum Status {
    /// A gateway that is closed and for which there is no timeline to open.
    ///
    /// **Note:** the definition of "open" here means transitioning to any other
    /// [`Gateway`](crate::Gateway) type.
    IndefinitelyClosed,

    /// A gateway that is closed while awaiting publication.
    AwaitingPublication {
        /// If known, the ISO 8601 formatted, UTC-based date and time when the
        /// the resource will become available.
        ///
        /// This field is intended to indicate to the consumer that they should
        /// retry their request on or after the listed time to gain an updated
        /// gateway definition.
        available_at: Option<DateTime<Utc>>,
    },

    /// A gateway that is closed while awaiting publication.
    Embargoed {
        /// The ISO 8601 formatted, UTC-based date and time when the the
        /// resource will become available.
        ///
        /// This field is intended to indicate to the consumer that they should
        /// retry their request on or after the listed time to gain an updated
        /// gateway definition.
        ///
        /// **Note:** for the gateway to have a kind of [`Status::Embargoed`],
        /// by definition, a date at which the resource becomes available _must_
        /// be known. If a date is not known, then the resource does not fit the
        /// API's definition of "embargoed".
        available_at: DateTime<Utc>,
    },
}
