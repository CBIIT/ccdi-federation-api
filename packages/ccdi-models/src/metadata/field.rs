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

use crate::metadata::field;

/// A metadata field.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq, ToSchema)]
#[serde(untagged)]
#[schema(as = field::UnharmonizedField)]
pub enum UnharmonizedField {
    /// An owned field.
    Owned(field::owned::Field),

    /// An unowned field.
    Unowned(field::unowned::Field),
}
