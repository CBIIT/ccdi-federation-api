//! Referenced identifiers for [`Samples`](crate::Sample).

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::sample::identifier::linked;
use crate::sample::identifier::unlinked;

/// A referenced identifier for a [`Sample`](crate::Sample).
///
/// A referenced identifier is a reference to either an identifier whose owner is known
/// and operates an authoritative federation server containing that identifier (i.e., a
/// [linked identifier](linked::Identifier)) _or_ a reference to an identifier that is
/// generally known to be associated with the sample but does not have an associated
/// server that asserts ownership of the identifier (i.e., an [unlinked
/// identifier](unlinked::Identifier)).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(tag = "type")]
#[schema(as = models::sample::identifier::referenced::Identifier)]
pub enum Identifier {
    /// A linked identifier.
    ///
    /// Linked identifiers are identifiers that are able to be linked back to servers
    /// within the federated ecosystem (i.e., the server that owns this identifier
    /// within the ecosystem is known).
    #[schema(value_type = models::sample::identifier::linked::Identifier)]
    Linked(linked::Identifier),

    /// An unlinked identifier.
    ///
    /// This represents an arbitrary identitier that cannot be linked to any source
    /// server in the broader federated ecosystem. There are no restricted values for
    /// this identifier.
    #[schema(value_type = models::sample::identifier::unlinked::Identifier)]
    Unlinked(unlinked::Identifier),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::Linked(identifier) => {
                write!(f, "Linked Sample {{ inner: {} }}", identifier)
            }
            Identifier::Unlinked(identifier) => {
                write!(f, "Unlinked Sample {{ inner: {} }}", identifier)
            }
        }
    }
}
