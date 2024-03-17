//! Unlinked identifiers for [`Subjects`](crate::Subject).

use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// An unlinked identifier for a [`Subject`](crate::Subject).
///
/// This represents an arbitrary identitier that cannot be linked to any source server
/// in the broader federated ecosystem. There are no restricted values for this
/// identifier.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[schema(as = models::subject::identifier::unlinked::Identifier)]
pub struct Identifier {
    name: String,
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
