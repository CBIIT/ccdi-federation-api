//! Harmonized metadata field descriptions for samples.

use ccdi_cde as cde;

use cde::CDE;

use crate::metadata::field::description;
use crate::metadata::field::description::r#trait::Description as _;
use crate::metadata::field::description::Harmonized;

/// Gets the harmonized fields for samples.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        cde::v1::sample::DiseasePhase::description(),
        cde::v2::sample::TissueType::description(),
        cde::v1::sample::TumorClassification::description(),
    ]
}

impl description::r#trait::Description for cde::v1::sample::DiseasePhase {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "disease_phase",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v2::sample::TissueType {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "tissue_type",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

impl description::r#trait::Description for cde::v1::sample::TumorClassification {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "tumor_classification",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_of_the_descriptions_unwrap() {
        // Because each of the field descriptions are instantiated when this
        // array is constructed, we can simply use this function to test that
        // all of them unwrap successfully.
        get_field_descriptions();
    }
}
