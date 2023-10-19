//! A metadata field.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod description;

pub mod owned;
pub mod unowned;

pub use description::Description;

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
