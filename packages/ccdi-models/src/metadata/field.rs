//! A metadata field.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod description;
pub mod owned;
pub mod unowned;

pub use description::Description;

pub use owned::Identifier;
pub use unowned::Ethnicity;
pub use unowned::Race;
pub use unowned::Sex;

/// A metadata field.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
#[serde(untagged)]
pub enum Field {
    /// An owned field.
    Owned(owned::Field),

    /// An unowned field.
    Unowned(unowned::Field),
}
