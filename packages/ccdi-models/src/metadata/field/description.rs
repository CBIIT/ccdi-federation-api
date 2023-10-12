//! Metadata field descriptions.

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

mod harmonized;
mod unharmonized;

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

    use cde::Standard;
    use cde::Url;
    use cde::CDE;

    use crate::metadata::field::description::Harmonized;

    /// A trait to get a [`Description`] for a [`CDE`].
    pub trait Description
    where
        Self: CDE,
    {
        /// Gets the [`Description`].
        fn description() -> super::Description;
    }

    impl Description for cde::v1::Sex {
        fn description() -> super::Description {
            super::Description::Harmonized(Harmonized::new("sex", Self::standard(), Self::url()))
        }
    }

    impl Description for cde::v1::Race {
        fn description() -> super::Description {
            super::Description::Harmonized(Harmonized::new("race", Self::standard(), Self::url()))
        }
    }

    impl Description for cde::v2::Ethnicity {
        fn description() -> super::Description {
            super::Description::Harmonized(Harmonized::new(
                "ethnicity",
                Self::standard(),
                Self::url(),
            ))
        }
    }

    impl Description for cde::v1::Identifier {
        fn description() -> super::Description {
            super::Description::Harmonized(Harmonized::new(
                "identifiers",
                Self::standard(),
                Self::url(),
            ))
        }
    }
}
