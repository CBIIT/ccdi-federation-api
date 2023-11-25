//! Harmonized metadata field descriptions for subjects.

use ccdi_cde as cde;

use cde::CDE;

use crate::metadata::field::description;
use crate::Url;

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
            String::from("sex"),
            entity.standard().to_string(),
            Url::from(entity.url().clone()),
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
            String::from("race"),
            entity.standard().to_string(),
            Url::from(entity.url().clone()),
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
            String::from("ethnicity"),
            entity.standard().to_string(),
            Url::from(entity.url().clone()),
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
            String::from("identifiers"),
            entity.standard().to_string(),
            Url::from(entity.url().clone()),
            entity,
            members,
        ))
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::metadata::field::description::Description;
    use crate::HARMONIZED_KEY_REGEX;

    use super::*;

    #[test]
    fn all_of_the_descriptions_unwrap() {
        // Because each of the field descriptions are instantiated when this
        // array is constructed, we can simply use this function to test that
        // all of them unwrap successfully.
        get_field_descriptions();
    }

    #[test]
    fn all_of_the_harmonized_keys_conform_to_the_harmonized_key_regex() {
        let regex = Regex::new(HARMONIZED_KEY_REGEX).unwrap();

        for field in get_field_descriptions() {
            let path = match field {
                Description::Harmonized(description) => description.path,
                // This test is only concerned with harmonized fields, so we can
                // skip non-harmonized ones.
                Description::Unharmonized(_) => continue,
            };

            assert!(regex.is_match(path.as_str()))
        }
    }
}
