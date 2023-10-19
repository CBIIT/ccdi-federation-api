//! Harmonized metadata field descriptions for subjects.

use ccdi_cde as cde;

use cde::CDE;

use crate::metadata::field::description;

use crate::metadata::field::description::r#trait::Description;
use crate::metadata::field::description::Harmonized;

/// Gets the harmonized fields for subjects.
pub fn get_field_descriptions() -> Vec<description::Description> {
    vec![
        cde::v1::subject::Sex::description(),
        cde::v1::subject::Race::description(),
        cde::v2::subject::Ethnicity::description(),
        cde::v1::subject::Identifier::description(),
    ]
}

impl Description for cde::v1::subject::Sex {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "sex",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

impl Description for cde::v1::subject::Race {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "race",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

impl Description for cde::v2::subject::Ethnicity {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "ethnicity",
            entity.standard().to_string(),
            entity.url().to_string(),
            entity,
            members,
        ))
    }
}

impl Description for cde::v1::subject::Identifier {
    fn description() -> description::Description {
        // SAFETY: these two unwraps are tested statically below in the test
        // that constructs the description using `get_fields()`.
        let entity = Self::entity().unwrap();
        let members = Self::members().unwrap();

        description::Description::Harmonized(Harmonized::new(
            "identifiers",
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
