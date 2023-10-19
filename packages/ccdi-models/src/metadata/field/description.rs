//! Metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub mod harmonized;
pub mod unharmonized;

pub use harmonized::Harmonized;
pub use unharmonized::Unharmonized;

/// A description for a metadata field.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
#[schema(as = models::metadata::field::Description)]
pub enum Description {
    /// A harmonized metadata field description.
    #[schema(value_type = models::metadata::field::description::Harmonized)]
    Harmonized(Harmonized),

    /// An unharmonized metadata field description.
    #[schema(value_type = models::metadata::field::description::Unharmonized)]
    Unharmonized(Unharmonized),
}

/// Traits related to a [`Description`].
pub mod r#trait {
    use ccdi_cde as cde;

    use cde::CDE;

    /// A trait to get a [`Description`] for a [`CDE`].
    pub trait Description
    where
        Self: CDE + Sized,
    {
        /// Gets the [`Description`].
        fn description() -> super::Description;
    }
}
